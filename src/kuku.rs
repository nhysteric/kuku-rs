use rand_core::RngCore;
use rand_pcg::Pcg64Mcg;
use std::fmt::Debug;
use std::mem::swap;
use std::time::UNIX_EPOCH;

use crate::{locfunc::LocFunc, ItemType, TableSizeType};

#[derive(Debug, Clone, PartialEq)]
pub struct KuKuTable {
    table_size: TableSizeType,
    stash_size: TableSizeType,
    loc_func_count: u32,
    loc_func_seed: u128,
    max_probe: u64,
    empty_item: ItemType,
    table: Vec<ItemType>,
    stash: Vec<ItemType>,
    temp: Vec<usize>,
    funcs: Vec<LocFunc>,
    inserted_items: u32,
}

impl KuKuTable {
    pub fn new(
        table_size: TableSizeType,
        stash_size: TableSizeType,
        loc_func_count: u32,
        loc_func_seed: u128,
        max_probe: u64,
        empty_item: ItemType,
    ) -> KuKuTable {
        KuKuTable {
            table_size,
            stash_size,
            loc_func_count,
            max_probe,
            inserted_items: 0,
            loc_func_seed,
            table: vec![empty_item; table_size as usize],
            empty_item,
            temp: Vec::with_capacity(stash_size as usize),
            stash: Vec::with_capacity(loc_func_count as usize),
            funcs: generate_loc_funcs(loc_func_count, loc_func_seed, table_size),
        }
    }

    pub fn insert(&mut self, item: &ItemType) -> Result<(), &str> {
        if self.query(item).is_some() {
            return Ok(());
        }
        if self.inserted_items == self.table_size + self.stash_size {
            return Err("Table is full");
        }
        if *item == self.empty_item {
            return Ok(());
        }
        let mut pcg = Pcg64Mcg::new(
            match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_nanos(),
                Err(_) => 0,
            },
        );
        let mut insert_item = *item;
        for _ in 0..self.max_probe {
            self.temp.clear();
            for f in &mut self.funcs {
                let i = f.hash(item);
                if self.table[i as usize] == self.empty_item {
                    self.table[i as usize] = insert_item;
                    self.inserted_items += 1;
                    return Ok(());
                } else {
                    self.temp.push(i as usize);
                }
            }
            let random_index = self.temp[(pcg.next_u32() % self.loc_func_count) as usize];
            swap(&mut self.table[random_index], &mut insert_item);
        }
        if self.stash.len() < self.stash_size as usize {
            self.stash.push(insert_item);
            self.inserted_items += 1;
            return Ok(());
        }
        Err("Insert failed")
    }

    pub fn query(&mut self, item: &ItemType) -> Option<(bool, usize)> {
        for f in &mut self.funcs {
            let i = f.hash(item);
            if self.table[i as usize] == *item {
                return Some((true, i as usize));
            }
        }
        self.stash
            .iter()
            .position(|x| *x == *item)
            .map(|position| (false, position))
    }

    pub fn fill_rate(&self) -> f32 {
        self.inserted_items as f32 / (self.table_size + self.stash_size) as f32
    }

    pub fn get_table(&self) -> &Vec<ItemType> {
        &self.table
    }

    pub fn get_stash(&self) -> &Vec<ItemType> {
        &self.stash
    }
}

pub fn generate_loc_funcs(
    loc_func_count: u32,
    loc_func_seed: u128,
    table_size: TableSizeType,
) -> Vec<LocFunc> {
    let mut funcs = Vec::with_capacity(loc_func_count as usize);
    for i in 0..loc_func_count {
        funcs.push(LocFunc::new(table_size, loc_func_seed + i as u128));
    }
    funcs
}
