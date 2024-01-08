import hilog from '@ohos.hilog';
import { RecordInput, RecordItem, Footer } from './component';
import { Todo } from './module'

@Entry
@Component
struct Index {
  @State @Watch('onChange') record: Todo[] = [];
  @State restCount: number = 0;
  @State completeCount: number = 0;
  @State currentType: string = 'all';
  @State renderRecord: Todo[] = [];

  onChange() {
    const r = this.record.filter(i => i.status === 0 && i.isDelete === 0);
    const c = this.record.filter(i => i.status === 1 && i.isDelete === 0);
    this.restCount = r.length;
    this.completeCount = c.length;
    this.renderRecord = this.record.filter(i => i.isDelete === 0);
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
  onTopToggle = (isOn: boolean) => {
    hilog.info(0x0000, 'testTag', `${isOn}`);
    this.record = this.record.map(i => {
      if (i.isDelete === 0) {
        i.status = isOn ? 0 : 1;
      }
      return i;
    })
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