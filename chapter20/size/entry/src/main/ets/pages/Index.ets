import hilog from '@ohos.hilog';
import testNapi from 'libentry.so';
import size from 'libsize.so';

@Entry
@Component
struct Index {

  build() {
    Row() {
      Column() {
        Text('cpp')
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            hilog.info(0x0000, 'testTag', 'Test NAPI 2 + 3 = %{public}d', testNapi.add(2, 3));
          })
        Text('Rust')
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            const result = size.add(1,2);
            hilog.info(0x0000, 'testTag', 'Test NAPI 2 + 3 = %{public}d', result);
          })
      }
      .width('100%')
    }
    .height('100%')
  }
}
