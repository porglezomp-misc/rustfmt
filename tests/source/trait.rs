// Test traits

trait Foo {
    fn bar(x: i32   ) -> Baz<   U> {       Baz::new()
  }

    fn baz(a: AAAAAAAAAAAAAAAAAAAAAA,
b: BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB)
-> RetType;

    fn foo(a: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, // Another comment
b: BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB)
           -> RetType              ; // Some comment

    fn baz(&mut self        ) -> i32          ;

fn increment(&     mut self, x: i32         );

    fn read(&mut self,          x: BufReader<R> /* Used to be MemReader */)
    where R: Read;
}

pub trait WriteMessage {
    fn write_message  (&mut self, &FrontendMessage) ->   io::Result<()>;
}

trait Runnable {
    fn handler(self: & Runnable   );
}

trait TraitWithExpr {
    fn fn_with_expr(x: [i32;       1]);
}

trait Test {
    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error>;
}

trait T<> {}

trait Foo { type Bar: Baz; type Inner: Foo   = Box< Foo >; }

trait ConstCheck<T>:Foo   where   T: Baz { 
        const   J:   i32;
}

trait Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttt<T> 
    where T: Foo {}

trait Ttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt<T> where T: Foo {}

trait FooBar<T> : Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt where J: Bar { fn test(); }

trait WhereList<T, J> where T: Foo, J: Bar {}

