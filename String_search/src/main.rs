use std::str::pattern::Searcher;

fn main() {
    let paragraph = String::from("A quick brown fox jumps over the lazy dog. 
                                    A quick brown fox jumps over the lazy dog. 
                                    A quick brown fox jumps over the lazy dog.
                                    A quick brown fox jumps over the lazy dog. 
                                    A quick brown fox jumps over the lazy dog.");
    let fox = paragraph.Searcher::next("fox");
  
}
