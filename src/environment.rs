use super::expr_interpret::Result;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use indextree::Arena;
use indextree::NodeId;
use std::collections::HashMap;
// TODO
//use std::rc::Rc;

// TODO: Rc<Literal>
pub type Environment = HashMap<String, Literal>;

#[derive(Debug)]
struct EnvironmentNode {
    map: Environment,
    keep_alive: bool,
}

// invariants:
// - cur points to a valid node, or it is None
#[derive(Debug)]
pub struct EnvironmentTree {
    tree: Arena<EnvironmentNode>,
    nid: Option<NodeId>,
}

impl EnvironmentTree {
    pub fn new() -> EnvironmentTree {
        let mut et = EnvironmentTree {
            tree: Arena::new(),
            nid: None,
        };
        et.push(Environment::new());
        et
    }

    pub fn push(&mut self, env: Environment) {
        let node = EnvironmentNode {
            map: env,
            keep_alive: false,
        };
        let child = self.tree.new_node(node);

        if let Some(parent) = self.nid {
            parent.append(child, &mut self.tree);
        }
        self.nid = Some(child);
    }

    // conditionally pop based on keep_alive flags
    pub fn pop(&mut self) {
        if let Some(nid) = self.nid {
            let n = &self.tree[nid];
            self.nid = n.parent();
            if !n.get().keep_alive {
                nid.remove_subtree(&mut self.tree)
            }
        }
    }

    // fn declaration
    // mark a certain branch as alive
    // recurse until it hits an marked node
    pub fn keep_branch(&mut self) -> Option<NodeId> {
        let nid = self.nid?;
        let mut n = &mut self.tree[nid];
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
        Some(nid)
    }

    // fn call
    // pre-conditions:
    // - tree is not empty
    // - new_id is valid
    pub fn checkout(&mut self, new_nid: NodeId) -> NodeId {
        let nid = self
            .nid
            .expect("Cannot checkout when EnvironmentTree is empty");
        self.tree.get(new_nid).expect("Invalid NodeId");
        self.nid = Some(new_nid);
        nid
    }

    // operations
    // TODO: resolver

    // TODO: Rc<Literal>
    // TODO: error handling
    pub fn get(&mut self, name: &str) -> Result<&Literal> {
        if let Some(nid) = self.nid {
            if let Some(tid) = self.find(nid, name) {
                if let Some(value) = self.tree[tid].get().map.get(name) {
                    return Ok(value);
                }
            }
        }

        Err(RuntimeError {
            msg: format!("Undefined variable '{}'", name),
        })
    }

    // TODO: Literal
    // TODO: error handling
    pub fn assign(&mut self, name: &str, value: Literal) -> Result<()> {
        if let Some(nid) = self.nid {
            if let Some(tid) = self.find(nid, name) {
                if let Some(value_ref) = self.tree[tid].get_mut().map.get_mut(name) {
                    *value_ref = value;
                    return Ok(());
                }
            }
        }

        Err(RuntimeError {
            msg: format!("Undefined variable '{}'", name),
        })
    }

    // pre-conditions:
    // - tree is not empty
    pub fn define(&mut self, name: String, value: Literal) {
        let nid = self
            .nid
            .expect("Cannot define variables in an empty EnvironmentTree");
        self.tree[nid].get_mut().map.insert(name, value);
    }

    fn find(&mut self, id: NodeId, name: &str) -> Option<NodeId> {
        id.ancestors(&self.tree)
            .find(|&aid| self.tree[aid].get().map.contains_key(name))
    }
}
