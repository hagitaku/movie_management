import style from "../css/register.module.css";
export default function register(){
    async function postRegisterForm(formData : any) {
        'use server'
        var movieTitle = formData.get("movie_title");
        var movieContents = formData.get("movie_contents");
        var movieMemo = formData.get("movie_memo");
        console.log("Title:",movieTitle);
        console.log("Contents:",movieContents);
        console.log("Memo:",movieMemo);
    }
    return (
        <main className={style.main}>
            <form className={style.form_group} action={postRegisterForm}>
                <div className={style.form_group}>
                    <label htmlFor="name">映画タイトル</label>
                    <div className={style.form_input}>
                        <input name="movie_title" type="text"></input>
                    </div>
                </div>
                <div className={style.form_group}>
                    <label htmlFor="name">映画情報</label>
                    <div className={style.form_input}>
                        <textarea name="movie_contents" rows={6} cols={40}></textarea>
                    </div>
                </div>
                <div className={style.form_group}>
                    <label htmlFor="name">メモ</label>
                    <div className={style.form_input}>
                        <textarea name="movie_memo" rows={6} cols={40}></textarea>
                    </div>
                </div>
                <div className={style.form_btn}>
                    <button id="register_btn" type="submit">登録</button>
                </div>
            </form>
        </main>
    )
}