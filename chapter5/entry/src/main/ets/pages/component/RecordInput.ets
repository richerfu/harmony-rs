@Preview
@Component
export struct RecordInput {
  @State value: string = '';
  @Link restCount: number;
  @Link completeCount: number;
  public onFinish: (content: string) => void;
  public onToggle: (isOn: boolean) => void;

  @Styles focusStyle() {
    .backgroundColor("white")
  }

  build() {
    Row() {
      Toggle({ type: ToggleType.Checkbox, isOn: this.restCount === 0 && this.completeCount !== 0 })
        .padding(0)
        .margin({
          top: 0, bottom: 0, left: 0, right: 20
        })
        .onClick(() => {
          this.onToggle(this.restCount === 0);
        })
      TextInput({ placeholder: 'What needs to be done?', text: this.value })
        .borderRadius(0)
        .backgroundColor('white')
        .width('90%')
        .stateStyles({
          focused: this.focusStyle,
          clicked: this.focusStyle,
          pressed: this.focusStyle
        })
        .onChange(e => {
          this.value = e;
        })
        .onSubmit(() => {
          this.onFinish(this.value);
          this.value = '';
        })
    }
    .padding(20)
    .backgroundColor('white')
    .margin({
      left: 20,
      right: 20,
      top: 20
    })
  }
}
