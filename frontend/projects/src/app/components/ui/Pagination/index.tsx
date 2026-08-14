import { useEffect, useState } from "react";

import { PaginationProps } from "@/schema/pagination";

import style from "./style.module.css";

const Pagination = ({
  currentPage,
  setCurrentPage,
  totalPage,
}: PaginationProps) => {
  const [pageNumberList, setPageNumberList] = useState<Array<number>>([]);
  useEffect(() => {
    setPageNumberList([...Array(totalPage)].map((_, i) => i + 1));
  }, []);
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
            onClick={() => setCurrentPage(page)}
          >
            {page}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default Pagination;
