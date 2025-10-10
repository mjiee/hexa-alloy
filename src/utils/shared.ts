export interface Response<R> {
  code?: number;
  message?: string;
  result?: R;
}

export interface QueryResult<T> {
  data?: T[];
  total: number;
}

export interface Pagination {
  page?: number;
  size?: number;
}
