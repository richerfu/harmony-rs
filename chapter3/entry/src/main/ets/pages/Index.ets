import router from '@ohos.router'
@Entry
@Component
struct Index {
  @State message: string = 'Hello World'

  build() {
    Row() {
      Column() {
        Button("CustomBuiltInComponentStyle").onClick(() => {
          router.pushUrl({
            url: 'pages/CustomBuiltInComponentStyle'
          })
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}