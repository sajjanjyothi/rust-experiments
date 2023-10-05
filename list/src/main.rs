
struct Node{
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut head = Node{
        value: 0,
        next: Some(Box::new(Node{
            value: 1,
            next: Some(Box::new(Node{
                value: 2,
                next: Some(Box::new(Node{
                    value: 3,
                    next: None,
                })),
            })),
        })),
    };

    let mut traversed_node = Vec::new();
    push_new(&mut head, 4);
    push_new(&mut head, 5);
    traverse_nodes(&head, &mut traversed_node);
    println!("{:?}", traversed_node);
}

fn push_new(node: &mut Node, value: i32){
    if let Some(next) = &mut node.next{
        push_new(next, value);
    }else{
        node.next = Some(Box::new(Node{
            value,
            next: None,
        }));
    }
}

fn push_list(node: &mut Node, list: Vec<i32>){
    for value in list{
        push_new(node, value);
    }
}

fn traverse_nodes(node: &Node, traversed_node: &mut Vec<i32>){
    traversed_node.push(node.value);
    if let Some(next) = &node.next{
        traverse_nodes(next, traversed_node);
    }
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_traverse_nodes(){
        let mut head = Node{
            value: 0,
            next: Some(Box::new(Node{
                value: 1,
                next: Some(Box::new(Node{
                    value: 2,
                    next: Some(Box::new(Node{
                        value: 3,
                        next: None,
                    })),
                })),
            })),
        };

        push_new(&mut head, 4);
        push_list(&mut head, vec![5, 6, 7]);
        let mut traversed_node = Vec::new();
        traverse_nodes(&head, &mut traversed_node);
        assert_eq!(traversed_node, vec![0, 1, 2, 3,4,5,6,7]);
    }
}