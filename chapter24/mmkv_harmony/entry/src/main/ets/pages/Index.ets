import hilog from '@ohos.hilog';
import testNapi from 'libentry.so'

@Entry
@Component
struct Index {
  @State message: string = 'Hello World'

  build() {
    Row() {
      Column() {
        Text(this.message)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            testNapi.mmkvInit();
            // 以下是mmkv的API 复用同一个类型声明
            // testNapi.init("/data/storage/el2/base/haps/entry/files/mmkv");
            // testNapi.setFloat(3.14,"float");
          })
      }
      .width('100%')
    }
    .height('100%')
  }
}
