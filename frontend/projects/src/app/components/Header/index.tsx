import style from "./style.module.css";

const Header = () =>{
    return (
        <header className={style["header-field"]}>
            <h1 className={style["header-login-txt"]}>
                login
            </h1>
        </header>
    )
}
export default Header;