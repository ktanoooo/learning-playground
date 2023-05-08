fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // ========================================================
    // フィールドの更新
    // ========================================================
    change_field();

    fn change_field() {
        // 後から変更する場合はインスタンす全体が可変でないといけない(mut)
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        println!("user1.email: {}", user1.email);
        user1.email = String::from("anotheremail@example.com");
        println!("user1.email: {}", user1.email);
    }

    // ========================================================
    // jsのオブジェクトのように省略できる
    // ===============p=========================================
    let user = create_user(
        String::from("create_user_email"),
        String::from("create_user_username"),
    );
    println!("user: {:?}", user);

    fn create_user(email: String, username: String) -> User {
        // フィールド初期化省略記法
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // ========================================================
    // 構造体更新記法p
    // ========================================================

    let user2 = User {
        email: String::from("user2@email.com"),
        username: String::from("user2"),
        active: true,
        sign_in_count: 1,
    };

    // 明示的にセットされていないフォール度が与えられたインスタンスのフィールドと同じになるようなる
    let user3 = User {
        email: String::from("user3@email.com"),
        username: String::from("user3"),
        ..user2
    };

    println!("user3: {:?}", user3);

    // ========================================================
    //　タプル構造体
    // ========================================================
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black.0: {:?}", black);
}
