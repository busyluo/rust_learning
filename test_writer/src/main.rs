struct Foo {
}

trait AsyncWrite {}

trait AsyncWriteExt: AsyncWrite {
    fn write(&self) {
        println!("write.");
    }
}

impl AsyncWrite for Foo {

}

impl<W: AsyncWrite> AsyncWriteExt for W {}

fn main() {
    let foo = Foo {};
    foo.write();
}