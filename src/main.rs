extern crate hyper;
#[macro_use]
extern crate string_cache;
extern crate html5ever;
extern crate tendril;
use hyper::Client;
use tendril::stream::TendrilSink;

fn count_divs(hdl: &html5ever::rcdom::Handle) -> isize {
   use html5ever::rcdom::NodeEnum::Element;
   let nodep = hdl.borrow();
   let mut divs = 0;

   if let Element(ref qname, _, _) = nodep.node {
      if qname.local == atom!("div") {
         divs += 1;
         println!("{:?}", qname);
      }
   }
   for child in &nodep.children {
      divs += count_divs(child);
   }
   divs
}

fn main() {
   //println!("Hello, world!");
   let client = Client::new();

   let mut res = client.get("https://en.wikipedia.org").send().unwrap();
   assert_eq!(res.status, hyper::Ok);
   let dom = html5ever::rcdom::RcDom::default();
   let parser = html5ever::parse_document(dom, html5ever::ParseOpts::default());
   let parserb = parser.from_bytes(html5ever::driver::BytesOpts::default());
   let outp = parserb.read_from(&mut res).unwrap();
   println!("{:?}", count_divs(&outp.document));
}
