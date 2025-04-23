# public routes

POST      /user
POST      /login

# private routes

GET       /user                        user profile page
GET       /user/:id                    other user's profile page
PUT       /user/:id
DELETE    /user/:id
POST      /user/friend/:id
DELETE    /user/friend/:id
GET       /compare/:id
POST      /logout

GET       /item/:id
GET       /cuisine/:id

GET       /prefs
POST      /prefs
GET       /prefs/:id
PUT       /prefs/:id
DELETE    /prefs/:id

GET       /event
POST      /event
GET       /event/:id
PUT       /event/:id
DELETE    /event/:id
POST      /event/:id/invite/:user_id
DELETE    /event/:id/invite/:user_id

# admin routes

POST      /item/
PUT       /item/:id
DELETE    /item/:id
PATCH     /item/:id                    enable/disable item

POST      /cuisine/
PUT       /cuisine/:id
DELETE    /cuisine/:id
PATCH     /cuisine/:id                 enable/disable item

