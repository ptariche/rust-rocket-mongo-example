use time;
use jwt;
use jwt::{encode, decode, Header, Algorithm};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
const JWT_SECRET: &[u8] = b"not_a_RealSecret$$$";

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Token {
  iat: i64,
  exp: i64,
  user: String
}

impl Token {
  fn is_expired(&self) -> bool {
    let now = time::get_time().sec;
    now >= self.exp
  }

  fn is_claimed_user(&self, claimed_user: String) -> bool {
    self.user == claimed_user
  }

  fn has_role(&self, role: &str) -> bool {
    self.roles.contains(&role.to_string())
  }
}


pub fn jwt_generate(user: String) -> String {
  let now = time::get_time().sec;
  let payload = Token {
      iat: now,
      exp: now + ONE_WEEK,
      user: user,
  };

  encode(Header::default(), &payload, JWT_SECRET).unwrap()
}

pub fn jwt_decode (token: String) -> jwt::TokenData<Token> {
  decode::<Token>(&token, JWT_SECRET, Algorithm::HS256).unwrap()
}
