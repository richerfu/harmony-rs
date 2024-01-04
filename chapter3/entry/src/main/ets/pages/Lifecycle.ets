import hilog from '@ohos.hilog'

@Component
struct CustomBusinessComponent {
  private privateField: number;
  public publicField: string;

  aboutToAppear() {
    hilog.info(0x0000, 'testTag', 'Sub Component aboutToAppear called');
  }

  aboutToDisappear() {
    hilog.info(0x0000, 'testTag', 'Sub Component aboutToDisappear called');
  }

  build() {
    Column() {
      Text(`${this.privateField}`)
      Text(`${this.publicField}`)
    }
  }
}


@Entry
@Component
struct Lifecycle {
  aboutToAppear() {
    hilog.info(0x0000, 'testTag', 'Page Root Component aboutToAppear called');
  }

  aboutToDisappear() {
    hilog.info(0x0000, 'testTag', 'Page Root Component aboutToDisappear called');
  }

  onPageShow() {
    hilog.info(0x0000, 'testTag', 'Page Root Component onPageShow called');

  }

  onPageHide() {
    hilog.info(0x0000, 'testTag', 'Page Root Component onPageHide called');
  }

  build() {
    Row() {
      Column() {
        CustomBusinessComponent()
      }
      .width('100%')
    }
    .height('100%')
  }
}