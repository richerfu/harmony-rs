@Entry
@Component
struct CustomBuiltInComponentStyle {
  @State message: string = 'Hello World'

  build() {
    Row() {
      Column({ space: 20 }) {
        Button('CustomBackgroundColor').backgroundColor('red');

        Button('ComplexCustomStyle', { type: ButtonType.Normal })
          .borderRadius(4)
          .fontColor('white')
          .backgroundColor('blue')
      }
      .width('100%')
    }
    .height('100%')
  }
}