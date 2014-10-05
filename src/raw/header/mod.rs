use std::str::{SendStr, Slice, Owned};
use std::collections::hashmap::{HashMap, Entry, Entries, MutEntries, MoveEntries};
use std::default::Default;
use std::mem;

#[deriving(Default, Show, Clone)]
pub struct RawHeaders {
    data: HashMap<SendStr, Vec<Vec<u8>>>
}

impl RawHeaders {
    pub fn new() { Default::default() }

    pub fn get(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.data.find(Slice(unsafe { mem::transmute(name) })).map(|raw| raw.as_slice())
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut [Vec<u8>]> {
        self.data.find_mut(Slice(unsafe { mem::transmute(name) })).map(|raw| raw.as_slice())
    }

    pub fn header(&mut self, name: String) -> Option<Entry<SendStr, Vec<Vec<u8>>>> {
        self.data.entry(Owned(name))
    }

    pub fn iter(&self) -> Entries<SendStr, Vec<Vec<u8>>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> MutEntries<SendStr, Vec<Vec<u8>>> {
        self.data.iter_mut()
    }

    pub fn into_iter(self) -> MoveEntries<SendStr, Vec<Vec<u8>>> {
        self.data.into_iter()
    }
}


