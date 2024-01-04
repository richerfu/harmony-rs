@Component
struct CustomBusinessComponent {
  private privateField: number;
  public publicField: string;

  build() {
    Column() {
      Text(`${this.privateField}`)
      Text(`${this.publicField}`)
    }
  }
}

@Entry
@Component
struct CustomComponentPage {
  build() {
    Row() {
      Column() {
        CustomBusinessComponent({ privateField: 1, publicField: 'test' })
        Row() {
          CustomBusinessComponent({ privateField: 1, publicField: 'test' })
        }
        .padding(10)
        .border({
          color: 'red',
          radius: 10,
          width: 1,
          style: BorderStyle.Solid
        })

      }.height('100%')
    }.width('100%')
  }
}