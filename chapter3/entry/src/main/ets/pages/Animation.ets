@Entry
@Component
struct Animation {
  @State translateX: number = 0;
  @State opacityValue: number = 1;

  build() {
    Row() {
      Column() {
        Text('Animation')
          .translate({
            x: this.translateX,
          })
          .opacity(this.opacityValue)
            // 定义动画效果
          .animation({ duration: 1000, curve: Curve.Ease })
          .fontColor('rgb(66, 184, 131)')


        Button('start').onClick(() => {
          this.translateX = 20;
          this.opacityValue = 0;
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}