use diesel::{
    helper_types::Limit,
    prelude::*,
    query_builder::InsertStatement,
    query_dsl::{
        methods::{FilterDsl, FindDsl, LimitDsl},
        LoadQuery,
    },
    RunQueryDsl,
};

pub(crate) struct BaseRepository<C, T> {
    table: T,
    pub conn: C,
}

impl<C, T> BaseRepository<C, T>
where
    T: Table + Clone + Copy,
    C: Connection,
{
    pub fn new(table: T, conn: C) -> Self {
        BaseRepository { conn, table }
    }

    pub fn create<I, Q, V>(&self, model: I) -> QueryResult<Q>
    where
        I: Insertable<T, Values = V>,
        InsertStatement<T, V>: LoadQuery<C, Q>,
    {
        model.insert_into(self.table).get_result::<Q>(&self.conn)
    }

    pub fn find_by_id<M, PK>(&self, pk: PK) -> QueryResult<M>
    where
        T: FindDsl<PK>,
        <T as FindDsl<PK>>::Output: LimitDsl + RunQueryDsl<C>,
        Limit<<T as FindDsl<PK>>::Output>: LoadQuery<C, M>,
    {
        self.table.find(pk).first(&self.conn)
    }

    pub fn find_one<M, P>(&self, predicate: P) -> QueryResult<M>
    where
        T: FilterDsl<P>,
        P: ExpressionMethods,
        <T as FilterDsl<P>>::Output: LoadQuery<C, M>,
    {
        self.table.filter(predicate).get_result::<M>(&self.conn)
    }

    pub fn find_all<M, P>(&self, predicate: P) -> QueryResult<Vec<M>>
    where
        T: FilterDsl<P>,
        P: ExpressionMethods,
        <T as FilterDsl<P>>::Output: LoadQuery<C, M>,
    {
        self.table.filter(predicate).get_results::<M>(&self.conn)
    }
}
