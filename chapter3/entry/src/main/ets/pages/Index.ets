import router from '@ohos.router'

@Entry
@Component
struct Index {
  @State message: string = 'Hello World'

  build() {
    Row() {
      Column({ space: 20 }) {
        Button("CustomBuiltInComponentStyle").onClick(() => {
          router.pushUrl({
            url: 'pages/CustomBuiltInComponentStyle'
          })
        })
        Button("CustomComponent").onClick(() => {
          router.pushUrl({
            url: 'pages/CustomComponent'
          })
        })
        Button('LifeCycle').onClick(() => {
          router.pushUrl({
            url: 'pages/Lifecycle'
          })
        })
        Button("Animation").onClick(() => {
          router.pushUrl({
            url: 'pages/Animation'
          })
        })
        Button('Animate').onClick(() => {
          router.pushUrl({
            url: 'pages/Animate'
          })
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}