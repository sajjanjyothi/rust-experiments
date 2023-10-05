
struct Node{
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main(){
    let mut root = Node{
        value: 0,
        left: None,
        right: None,
    };

    let left = Node{
        value: 1,
        left: None,
        right: None,
    };

    let right = Node{
        value: 2,
        left: None,
        right: None,
    };

    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    let mut traversed_node = Vec::new();
    pre_order(&root, &mut traversed_node);
    println!("{:?}", traversed_node);
}

fn pre_order(node: &Node, traversed_node : &mut Vec<i32>){
    traversed_node.push(node.value);
    if let Some(left) = &node.left{
        pre_order(left, traversed_node);
    }
    if let Some(right) = &node.right{
        pre_order(right, traversed_node);
    }
}

fn post_order(node: &Node, traversed_node : & mut Vec<i32>){
    if let Some(left) = &node.left{
        post_order(left, traversed_node);
    }
    if let Some(right) = &node.right{
        post_order(right, traversed_node);
    }
    traversed_node.push(node.value);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_pre_order(){
        let mut traversed_node = Vec::new();
        let mut root = Node{
            value: 0,
            left: None,
            right: None,
        };

        let left = Node{
            value: 1,
            left: None,
            right: None,
        };

        let right = Node{
            value: 2,
            left: None,
            right: None,
        };

        root.left = Some(Box::new(left));
        root.right = Some(Box::new(right));
        pre_order(&root, &mut traversed_node);
        assert_eq!(traversed_node, vec![0, 1, 2]);
    }

    #[test]
    fn test_post_order(){
        let mut traversed_node = Vec::new();
        let mut root = Node{
            value: 0,
            left: None,
            right: None,
        };

        let mut left = Node{
            value: 1,
            left: None,
            right: None,
        };

        let right = Node{
            value: 2,
            left: None,
            right: None,
        };

        let left_left = Node{
            value: 3,
            left: None,
            right: None,
        };
        left.left = Some(Box::new(left_left));
        root.left = Some(Box::new(left));
        root.right = Some(Box::new(right));
        post_order(&root, &mut traversed_node);
        assert_eq!(traversed_node, vec![3,1, 2, 0]);
    }
}