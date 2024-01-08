@Preview
@Component
export struct Footer {
  @Prop restCount: number;
  @Prop completeCount: number;
  @Link currentType: string;
  public clear: () => void;

  @Styles common() {
    .border({
      radius: 4,
      width: 1,
      color: "rgba(175, 47, 47, 0.2)"
    })
    .padding({
      top: 6,
      bottom: 6,
      left: 10,
      right: 10
    })
  }

  build() {
    Row({ space: 20 }) {
      Text(`${this.restCount} left`)
      Row({ space: 10 }) {
        if (this.currentType === 'all') {
          Text('All').common()
        } else {
          Text("All").onClick(() => {
            this.currentType = 'all';
          })
        }
        if (this.currentType === 'active') {
          Text('A').common()
        } else {
          Text("A").onClick(() => {
            this.currentType = 'active';
          })
        }
        if (this.currentType === 'complete') {
          Text('C').common()
        } else {
          Text("C").onClick(() => {
            this.currentType = 'complete';
          })
        }
      }

      if (this.completeCount > 0) {
        Text('Clear').onClick(() => {
          this.clear()
        })
      }
    }
    .justifyContent(FlexAlign.SpaceAround)
    .margin({
      left: 20,
      right: 20
    })
    .padding({ bottom: 20, top: 20 })
    .width('90%')
    .backgroundColor('white')
  }
}