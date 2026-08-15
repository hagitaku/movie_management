import { useEffect, useState } from "react";

import { PaginationProps } from "@/schema/pagination";

import style from "./style.module.css";

const Pagination = ({
  currentPage,
  handleClickPage,
  totalPage,
  maxPaginationItem,
}: PaginationProps) => {
  const [pageNumberList, setPageNumberList] = useState<Array<number>>([]);

  const generatePageNumberList = (currentPage: number, totalPage: number) => {
    const pageDiff = totalPage - currentPage + 1;
    if (pageDiff < maxPaginationItem) {
      return [...Array(maxPaginationItem)].map(
        (_, i) => totalPage - maxPaginationItem + i + 1,
      );
    }
    return [...Array(maxPaginationItem)].map((_, i) => currentPage + i);
  };

  useEffect(() => {
    setPageNumberList(generatePageNumberList(currentPage, totalPage));
  }, [totalPage, currentPage]);
  return (
    <div className={style["pagination-container"]}>
      <ul className={style["pagination-ul"]}>
        {pageNumberList.map((page) => (
          <li
            className={
              page === currentPage
                ? style["pagination-item-active"]
                : style["pagination-item"]
            }
            key={page}
            onClick={() => handleClickPage(page)}
          >
            {page}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default Pagination;
