import hilog from '@ohos.hilog';
import relationalStore from '@ohos.data.relationalStore';
import { RecordInput, RecordItem, Footer } from './component';
import { Todo } from './module'
import { STORE_CONFIG } from '../constants'

@Entry
@Component
struct Index {
  @State @Watch('onChange') record: Todo[] = [];
  @State restCount: number = 0;
  @State completeCount: number = 0;
  @State currentType: string = 'all';
  @State renderRecord: Todo[] = [];
  private store: relationalStore.RdbStore = null;

  onChange() {
    const r = this.record.filter(i => i.status === 0 && i.isDelete === 0);
    const c = this.record.filter(i => i.status === 1 && i.isDelete === 0);
    this.restCount = r.length;
    this.completeCount = c.length;
    this.renderRecord = this.record.filter(i => i.isDelete === 0);

    this.record.forEach(r => {
      this.updateStore(r);
    })
  }

  updateStore = async (record: Todo) => {
    let predicates = new relationalStore.RdbPredicates("todos");
    predicates.equalTo("id", record.id);
    const result = await this.store.query(predicates, ['id']);
    const valueBucket = {
      "id": record.id,
      "content": record.content,
      "status": record.status,
      "is_delete": record.isDelete
    };
    // 记录不存在
    if (result.rowCount === 0) {
      let c = await this.store.insert('todos', valueBucket);
      hilog.info(0x0000, 'testTag', `${c}`);
    } else {
      await this.store.update(valueBucket, predicates);
    }
  }
  onFinish = (content: string) => {
    hilog.info(0x0000, 'testTag', '%{public}s', content);
    this.record.push(new Todo(content));
  }
  onClear = () => {
    this.record = this.record.map(i => {
      if (i.status === 1) {
        i.isDelete = 1;
      }
      return i;
    })
  }

  // must add toggle to listen object link change,maybe a bug?
  onToggle = () => {
    this.onChange();
  }
  onDelete = () => {
    this.onChange();
  }
  getStoreRecords = (resultSet: relationalStore.ResultSet): Todo[] => {
    const result: Todo[] = [];
    while (resultSet.goToNextRow()) {
      const id = resultSet.getString(resultSet.getColumnIndex("id"));
      const content = resultSet.getString(resultSet.getColumnIndex("content"));
      const status = resultSet.getDouble(resultSet.getColumnIndex("status"));
      const isDelete = resultSet.getDouble(resultSet.getColumnIndex("is_delete"));
      result.push(new Todo(content, id, status, isDelete))
    }
    // 释放数据集的内存
    resultSet.close();
    return result;
  }
  onTopToggle = (isOn: boolean) => {
    hilog.info(0x0000, 'testTag', `${isOn}`);
    this.record = this.record.map(i => {
      if (i.isDelete === 0) {
        i.status = isOn ? 0 : 1;
      }
      return i;
    })
  }

  async aboutToAppear() {
    this.store = await relationalStore.getRdbStore(getContext(this), STORE_CONFIG);
    const result = await this.store.querySql("SELECT * FROM todos");
    const records = this.getStoreRecords(result);
    this.record = records;
  }

  build() {
    Row() {
      Column() {
        Text('Todos')
          .fontSize(40)
          .fontColor("rgba(175, 47, 47, 0.15)").margin(20)
        RecordInput({
          onFinish: this.onFinish,
          onToggle: this.onTopToggle,
          completeCount: $completeCount,
          restCount: $restCount
        })
        ForEach(this.renderRecord.filter(i => {
          if (this.currentType === 'active') {
            return i.status === 0;
          } else if (this.currentType === 'complete') {
            return i.status === 1;
          }
          return true;
        }), (item) => {
          RecordItem({ recordItem: item, onToggle: this.onToggle, onDelete: this.onDelete })
        })
        Footer({
          restCount: this.restCount,
          completeCount: this.completeCount,
          currentType: $currentType,
          clear: this.onClear
        })
      }
      .width('100%')
      .justifyContent(FlexAlign.Start)
      .backgroundColor("#f5f5f5")
    }
    .height('100%').alignItems(VerticalAlign.Top).backgroundColor("#f5f5f5")
  }
}