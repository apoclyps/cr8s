use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find_multiple(c: &PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustacean>(c)
    }

    pub fn find(c: &PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn create(c: &PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
    }

    pub fn save(c: &PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find_multiple(c: &PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(c)
    }

    pub fn find(c: &PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result::<Crate>(c)
    }

    pub fn create(c: &PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(c)
    }

    pub fn save(c: &PgConnection, id: i32, update_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::code.eq(update_crate.code.to_owned()),
                crates::name.eq(update_crate.name.to_owned()),
                crates::version.eq(update_crate.version.to_owned()),
                crates::description.eq(update_crate.description.to_owned()),
                crates::rustacean_id.eq(update_crate.rustacean_id.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c)
    }
}

pub struct UserRepository;

impl UserRepository {
    pub fn find(c: &PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }

    pub fn find_with_roles(c: &PgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(c)?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub fn find_by_username(c: &PgConnection, username: &String) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result::<User>(c)
    }

    pub fn create(
        c: &PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(&c, &role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let name: String = role_code.as_str().to_owned();
                    let new_role: NewRole = NewRole {
                        code: role_code,
                        name,
                    };

                    let role = RoleRepository::create(&c, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };
            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .execute(c)?;
        }

        Ok(user)
    }

    pub fn delete(c: &PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id))).execute(c)?;

        diesel::delete(users::table.find(id)).execute(c)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub fn find_by_user(c: &PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(user).get_results(c)?;

        Self::find_by_ids(
            &c,
            user_roles
                .iter()
                .map(|user_role: &UserRole| user_role.role_id)
                .collect(),
        )
    }

    pub fn find_by_ids(c: &PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).get_results(c)
    }

    pub fn find_by_code(c: &PgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table
            .filter(roles::code.eq(code))
            .get_result::<Role>(c)
    }

    pub fn create(c: &PgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
    }
}
