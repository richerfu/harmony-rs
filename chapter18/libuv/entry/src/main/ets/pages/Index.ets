import hilog from '@ohos.hilog';
import testNapi from 'libentry.so';

const fib = (n: number) => {
  if (n <= 1) {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
}

@Entry
@Component
struct Index {
  @State count: number = 0;

  build() {
    Row() {
      Column({ space: 20 }) {
        Text(`current count: ${this.count}`)
        Button("run update").onClick(() => {
          this.count += 1;
        })
        Button("run fib").onClick(() => {
          hilog.info(0x0000, 'testTag', 'fib %{public}d', fib(45));
        })
        Button("run native-fib").onClick(async () => {
          hilog.info(0x0000, 'testTag', 'native-fib %{public}d', await testNapi.asyncWork(45));
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
