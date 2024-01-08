import { Todo } from '../module'

@Preview
@Component
export struct RecordItem {
  @State editable: boolean = false;
  @ObjectLink recordItem: Todo;
  @State editValue: string = '';
  public onToggle: () => void;
  public onDelete: () => void;

  @Styles focusStyle() {
    .backgroundColor("white")
  }

  build() {
    Row() {
      Row() {
        Toggle({ type: ToggleType.Checkbox, isOn: this.recordItem.status === 1 })
          .margin({ right: 10 })
          .onClick(() => {
            this.recordItem.status = this.recordItem.status === 0 ? 1 : 0;
            this.onToggle();
          })
        if (!this.editable) {
          Text(this.recordItem.content)
            .textOverflow({ overflow: TextOverflow.Ellipsis })
            .maxLines(1)
            .margin({
              right: 30
            })
              // 双击事件
            .gesture(TapGesture({ count: 2 }).onAction(() => {
              this.editValue = this.recordItem.content;
              this.editable = true;
            }))
            .decoration({
              type: this.recordItem.status === 1 ? TextDecorationType.LineThrough : TextDecorationType.None
            })
            .fontColor(this.recordItem.status === 1 ? "#888" : "black")
        } else {
          TextInput({ text: this.editValue})
            .borderRadius(0)
            .stateStyles({
              focused: this.focusStyle,
              clicked: this.focusStyle,
              pressed: this.focusStyle
            })
            .width('90%')
            .onChange(e => {
              this.editValue = e;
            })
            .onBlur(() => {
              this.editable = false;
            })
            .onSubmit(() => {
              this.recordItem.content = this.editValue;
              this.editable = false;
            })
        }

      }.width(this.editable ? '100%' : '90%')

      if (!this.editable) {
        Image('assets/delete.png').width(20).height(20).margin({
          right: 20
        }).onClick(() => {
          this.recordItem.isDelete = 1;
          this.onDelete();
        })
      }
    }
    .margin({
      left: 20,
      right: 20
    })
    .padding(10)
    .justifyContent(FlexAlign.SpaceBetween)
    .backgroundColor('white')
  }
}