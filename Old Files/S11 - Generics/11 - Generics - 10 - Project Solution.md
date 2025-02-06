```rust
/*
Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.

Derive a ChatMessage struct with two fields: 'content'
and 'time'. The struct should define one generic type, T,
which will be the type of the 'content' field. 
The 'time' field should always be a String.
Derive a Debug implementation.

Add an impl block for ChatMessage structs whose T type 
is a DigitalContent enum. Define a `consume_entertainment` 
method that prints out the value of the `content` field in 
Debug format. For example, "Watching the AudioFile".

Add an impl block for ChatMessage structs with any type T.
Define a 'retrieve_time' method that returns a string.
It should return a clone of the `time` field from the struct.

In main, create a ChatMessage with 'content' set to a 
string slice.

Create another ChatMessage with 'content' set to a String.

Create another ChatMessage with 'content' set to a DigitalContent
variant.

Invoke the retrieve_time method on this ChatMessage and print
out the String's content.

Invoke the consume_entertainment method on the ChatMessage
storing a DigitalContent enum.
*/

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message = ChatMessage {
        content: "Hi, lol",
        time: String::from("2025-04-12"),
    };
    println!("{}", message.retrieve_time());

    let pizza = ChatMessage {
        content: String::from("What's your favorite pizza topping?"),
        time: String::from("2025-04-12"),
    };

    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2025-05-12"),
    };
    audio.consume_entertainment();
}

```
