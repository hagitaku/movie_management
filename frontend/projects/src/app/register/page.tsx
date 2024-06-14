import style from "../css/register.module.css";
export default function Register(){
    return (
        <main className={style.main}>
            <div className={style.container}>
                <div className={style.form_group}>
                    <label htmlFor="name" className="title">映画タイトル</label>
                    <div className={style.from_input}>
                        <input id="movie_title" type="text" className="title_input"></input>
                    </div>
                </div>
                <div className={style.form_group}>
                    <label htmlFor="name" className="contents">映画情報</label>
                    <div className={style.from_input}>
                        <textarea id="movie_contents" className="contents_input" rows={6} cols={40}></textarea>
                    </div>
                </div>
                <div className={style.form_group}>
                    <label htmlFor="name" className="memo">メモ</label>
                    <div className={style.from_input}>
                        <textarea id="movie_memo" className="memo_input" rows={6} cols={40}></textarea>
                    </div>
                </div>
            </div>
        </main>
    )
}