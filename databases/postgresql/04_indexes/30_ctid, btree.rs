#### INDEXES
// indexes needs for faster search
// this is btree
// index copyes feald value to itself
// do not create indexes on everything
// because if some value is updated, it cause uptation of a index field
// every index contains a pointer of phisical location of the value
// it is named ctid

#### CTID
// data phisical address

// under the hood in the pg
// data stores in pages
// pages has rows
// a page and a row is an address of the data

// if i select
SELECT *, ctid FORM my_table
// .... fields .... (0,1) // this row leaves in 0 page and 1 row
// .... fields .... (0,2) // this row leaves in 0 page and 2 row
// .... fields .... (0,3) // this row leaves in 0 page and 3 row

//! if a row will be updated, ctid can be changed

#### WHAT A BTREE
// you need to know where the index will work, and where will not

#### WHAT IS A PRIMARY KEY IN PG
// for example in mysql every table is a cluster index
// in pg a table lives in pages, that means all data lives in some sort of a heap
// and a table is not a cluster index like in the mysql db
// pg have no cluster indexes
// every index in pg is a secondary index
// primary key is a special tyge of a secondary index to (secondary index ++)
//! you should not set an index on a primary key field, because it creates at a table creation time
