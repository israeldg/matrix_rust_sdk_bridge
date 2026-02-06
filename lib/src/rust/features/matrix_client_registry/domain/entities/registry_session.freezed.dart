// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'registry_session.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$Credentials {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Credentials);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Credentials()';
}


}

/// @nodoc
class $CredentialsCopyWith<$Res>  {
$CredentialsCopyWith(Credentials _, $Res Function(Credentials) __);
}


/// Adds pattern-matching-related methods to [Credentials].
extension CredentialsPatterns on Credentials {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( Credentials_AccessToken value)?  accessToken,TResult Function( Credentials_UserPassword value)?  userPassword,required TResult orElse(),}){
final _that = this;
switch (_that) {
case Credentials_AccessToken() when accessToken != null:
return accessToken(_that);case Credentials_UserPassword() when userPassword != null:
return userPassword(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( Credentials_AccessToken value)  accessToken,required TResult Function( Credentials_UserPassword value)  userPassword,}){
final _that = this;
switch (_that) {
case Credentials_AccessToken():
return accessToken(_that);case Credentials_UserPassword():
return userPassword(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( Credentials_AccessToken value)?  accessToken,TResult? Function( Credentials_UserPassword value)?  userPassword,}){
final _that = this;
switch (_that) {
case Credentials_AccessToken() when accessToken != null:
return accessToken(_that);case Credentials_UserPassword() when userPassword != null:
return userPassword(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String field0)?  accessToken,TResult Function( String username,  String password)?  userPassword,required TResult orElse(),}) {final _that = this;
switch (_that) {
case Credentials_AccessToken() when accessToken != null:
return accessToken(_that.field0);case Credentials_UserPassword() when userPassword != null:
return userPassword(_that.username,_that.password);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String field0)  accessToken,required TResult Function( String username,  String password)  userPassword,}) {final _that = this;
switch (_that) {
case Credentials_AccessToken():
return accessToken(_that.field0);case Credentials_UserPassword():
return userPassword(_that.username,_that.password);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String field0)?  accessToken,TResult? Function( String username,  String password)?  userPassword,}) {final _that = this;
switch (_that) {
case Credentials_AccessToken() when accessToken != null:
return accessToken(_that.field0);case Credentials_UserPassword() when userPassword != null:
return userPassword(_that.username,_that.password);case _:
  return null;

}
}

}

/// @nodoc


class Credentials_AccessToken extends Credentials {
  const Credentials_AccessToken(this.field0): super._();
  

 final  String field0;

/// Create a copy of Credentials
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Credentials_AccessTokenCopyWith<Credentials_AccessToken> get copyWith => _$Credentials_AccessTokenCopyWithImpl<Credentials_AccessToken>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Credentials_AccessToken&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Credentials.accessToken(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Credentials_AccessTokenCopyWith<$Res> implements $CredentialsCopyWith<$Res> {
  factory $Credentials_AccessTokenCopyWith(Credentials_AccessToken value, $Res Function(Credentials_AccessToken) _then) = _$Credentials_AccessTokenCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Credentials_AccessTokenCopyWithImpl<$Res>
    implements $Credentials_AccessTokenCopyWith<$Res> {
  _$Credentials_AccessTokenCopyWithImpl(this._self, this._then);

  final Credentials_AccessToken _self;
  final $Res Function(Credentials_AccessToken) _then;

/// Create a copy of Credentials
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Credentials_AccessToken(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Credentials_UserPassword extends Credentials {
  const Credentials_UserPassword({required this.username, required this.password}): super._();
  

 final  String username;
 final  String password;

/// Create a copy of Credentials
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Credentials_UserPasswordCopyWith<Credentials_UserPassword> get copyWith => _$Credentials_UserPasswordCopyWithImpl<Credentials_UserPassword>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Credentials_UserPassword&&(identical(other.username, username) || other.username == username)&&(identical(other.password, password) || other.password == password));
}


@override
int get hashCode => Object.hash(runtimeType,username,password);

@override
String toString() {
  return 'Credentials.userPassword(username: $username, password: $password)';
}


}

/// @nodoc
abstract mixin class $Credentials_UserPasswordCopyWith<$Res> implements $CredentialsCopyWith<$Res> {
  factory $Credentials_UserPasswordCopyWith(Credentials_UserPassword value, $Res Function(Credentials_UserPassword) _then) = _$Credentials_UserPasswordCopyWithImpl;
@useResult
$Res call({
 String username, String password
});




}
/// @nodoc
class _$Credentials_UserPasswordCopyWithImpl<$Res>
    implements $Credentials_UserPasswordCopyWith<$Res> {
  _$Credentials_UserPasswordCopyWithImpl(this._self, this._then);

  final Credentials_UserPassword _self;
  final $Res Function(Credentials_UserPassword) _then;

/// Create a copy of Credentials
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? username = null,Object? password = null,}) {
  return _then(Credentials_UserPassword(
username: null == username ? _self.username : username // ignore: cast_nullable_to_non_nullable
as String,password: null == password ? _self.password : password // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
