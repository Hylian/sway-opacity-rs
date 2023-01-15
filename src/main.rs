use clap::Parser;
use i3ipc::{event::{Event, WindowEventInfo, inner::WindowChange}, reply::{Node, NodeType}, I3Connection, I3EventListener, Subscription};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0.8)]
    opacity: f64,
}
/*
struct IterNode(Node);

struct IntoIter {
    remaining: Vec<Node>,
}

impl IntoIterator for IterNode {
    type IntoIter = IntoIter;
    type Item = IterNode;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            remaining: self.0.nodes,
        }
    }
}

impl Iterator for IntoIter {
    type Item = IterNode;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.remaining.pop().and_then(|n| {
            self.remaining.extend(n.nodes);

            Some(IterNode(n.clone()))
        })
    }
}

fn set_unfocused_opacity(connection: &mut I3Connection) {
    let root = connection.get_tree().unwrap();
    let root_iter = IterNode(root).into_iter();
    let con_iter = root_iter.filter(|x| x.0.nodetype == NodeType::Con);
    for c in con_iter {
        let command = format!("[con_id={}] opacity set {}", c.0.id, 0.8);
        connection.run_command(&command).unwrap();
    }
}
*/

fn set_focused_opacity(connection: &mut I3Connection, container: Node, opacity: f64) {
    //set_unfocused_opacity(&mut connection);

    let command = format!("[app_id=\".*\" title=\"^(?!.*- YouTube - Google Chrome$).*$\"] opacity set {} \n", opacity);
    connection.run_command(&command).unwrap();

    let command = format!("[con_id=__focused__] opacity set {} \n", 1.0);
    connection.run_command(&command).unwrap();

    /* 
    let command = format!("[title=\".*- YouTube - Google Chrome\"] opacity set 1.0 \n");
    connection.run_command(&command).unwrap();
    */

    //println!("{}", command);
    //dbg!(container);
}

fn main() {
    let args = Args::parse();
    
    let mut connection = I3Connection::connect().unwrap();
    
    let mut listener = I3EventListener::connect().unwrap();
    listener.subscribe(&[Subscription::Window]).unwrap();
    
    let mut iter = listener.listen();
    
    loop {
        match iter.next() {
            Some(Ok(
                Event::WindowEvent(
                    WindowEventInfo{
                        change: WindowChange::Focus, container: c}))) => {
                //dbg!(&container);
                set_focused_opacity(&mut connection, c, args.opacity)
            },
            Some(Ok(_e)) => { continue },
            Some(Err(_e)) => { println!("error!"); std::process::exit(1); },
            None => { break }
        }
    }
}
