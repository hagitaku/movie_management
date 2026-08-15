export type PaginationProps = {
  currentPage: number;
  handleClickPage: (page: number) => void;
  totalPage: number;
  maxPaginationItem: number;
};
