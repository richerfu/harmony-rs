@Entry
@Component
struct Animate {
  @State translateX: number = 0;
  @State opacityValue: number = 1;

  build() {
    Row() {
      Column() {
        Text('Animate')
          .translate({
            x: this.translateX,
          })
          .opacity(this.opacityValue)
          .fontColor('rgb(66, 184, 131)')


        Button('start').onClick(() => {
          animateTo({ duration: 1000, curve: Curve.Ease }, () => {
            this.translateX = 20;
            this.opacityValue = 0;
          })
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}