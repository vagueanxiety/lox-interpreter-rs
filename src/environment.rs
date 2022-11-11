use super::expr_interpret::Result;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use super::token::Token;
use indextree::Arena;
use indextree::NodeId;
use std::collections::HashMap;
use std::rc::Rc;

pub type Environment = HashMap<String, Rc<Literal>>;

struct EnvironmentNode {
    map: Environment,
    keep_alive: bool,
}

// invariants:
// - cur points to a valid node, or it is None
pub struct EnvironmentTree {
    tree: Arena<EnvironmentNode>,
    global_nid: NodeId,
    nid: NodeId,
}

impl EnvironmentTree {
    pub fn new() -> EnvironmentTree {
        let mut tree = Arena::new();
        let global_nid = tree.new_node(EnvironmentNode {
            map: Environment::new(),
            keep_alive: true,
        });

        EnvironmentTree {
            tree,
            global_nid,
            nid: global_nid,
        }
    }

    pub fn push(&mut self, env: Environment) {
        let node = EnvironmentNode {
            map: env,
            keep_alive: false,
        };
        let child = self.tree.new_node(node);
        self.nid.append(child, &mut self.tree);
        self.nid = child;
    }

    // conditionally pop based on keep_alive flags and global_nid
    pub fn pop(&mut self) {
        if self.nid != self.global_nid {
            // make a copy before changing self.nid
            let nid = self.nid;
            let n = &self.tree[nid];
            self.nid = n
                .parent()
                .expect("Local EnvironmentNode must have a parent");
            if !n.get().keep_alive {
                nid.remove_subtree(&mut self.tree)
            }
        }
    }

    // fn declaration
    // mark a certain branch as alive
    // recurse until it hits an marked node
    pub fn keep_branch(&mut self) -> NodeId {
        let mut n = &mut self.tree[self.nid];
        loop {
            if n.get().keep_alive {
                break;
            } else {
                n.get_mut().keep_alive = true;
            }

            if let Some(pid) = n.parent() {
                n = &mut self.tree[pid];
            } else {
                break;
            }
        }
        self.nid
    }

    // fn call
    // pre-conditions:
    // - tree is not empty
    // - new_id is valid
    pub fn checkout(&mut self, new_nid: NodeId) -> NodeId {
        let nid = self.nid;
        self.tree.get(new_nid).expect("Invalid NodeId");
        self.nid = new_nid;
        nid
    }

    // operations
    // TODO: resolver
    pub fn get(&mut self, name: &Token) -> Result<&Rc<Literal>> {
        if let Some(tid) = self.find(self.nid, &name.lexeme) {
            if let Some(value) = self.tree[tid].get().map.get(&name.lexeme) {
                return Ok(value);
            }
        }

        Err(RuntimeError::new(
            name,
            &format!("Undefined variable '{}'", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Rc<Literal>) -> Result<()> {
        if let Some(tid) = self.find(self.nid, &name.lexeme) {
            if let Some(value_ref) = self.tree[tid].get_mut().map.get_mut(&name.lexeme) {
                *value_ref = value;
                return Ok(());
            }
        }

        Err(RuntimeError::new(
            name,
            &format!("Undefined variable '{}'", name.lexeme),
        ))
    }

    // pre-conditions:
    // - tree is not empty
    pub fn define(&mut self, name: String, value: Rc<Literal>) {
        self.tree[self.nid].get_mut().map.insert(name, value);
    }

    fn find(&mut self, id: NodeId, name: &str) -> Option<NodeId> {
        id.ancestors(&self.tree)
            .find(|&aid| self.tree[aid].get().map.contains_key(name))
    }
}
