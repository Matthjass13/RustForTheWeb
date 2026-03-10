from django.db import models

class User(models.Model):
    # Django will automatically handle the 'id' field as an AutoField (i32/Serial)
    name = models.CharField(max_length=255) # Matches String in Rust
    email = models.EmailField(unique=True)   # Matches String in Rust

    class Meta:
        # Crucial: This tells Django to use the table name from your init.sql/Rust app
        db_table = 'users' 
        
    def __str__(self):
        return self.name