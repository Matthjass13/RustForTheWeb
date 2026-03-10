from django.urls import path
from api import views

urlpatterns = [
    path('', views.index),                               # Route: /
    path('users', views.user_list_create),               # Route: /users
    path('users/<int:user_id>', views.user_detail),      # Route: /users/:id
]