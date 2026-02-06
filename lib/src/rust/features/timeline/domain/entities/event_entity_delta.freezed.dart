// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_entity_delta.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$EventDeltaEntity {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'EventDeltaEntity()';
}


}

/// @nodoc
class $EventDeltaEntityCopyWith<$Res>  {
$EventDeltaEntityCopyWith(EventDeltaEntity _, $Res Function(EventDeltaEntity) __);
}


/// Adds pattern-matching-related methods to [EventDeltaEntity].
extension EventDeltaEntityPatterns on EventDeltaEntity {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( EventDeltaEntity_PushFront value)?  pushFront,TResult Function( EventDeltaEntity_PushBack value)?  pushBack,TResult Function( EventDeltaEntity_Insert value)?  insert,TResult Function( EventDeltaEntity_Remove value)?  remove,TResult Function( EventDeltaEntity_Update value)?  update,TResult Function( EventDeltaEntity_Reset value)?  reset,required TResult orElse(),}){
final _that = this;
switch (_that) {
case EventDeltaEntity_PushFront() when pushFront != null:
return pushFront(_that);case EventDeltaEntity_PushBack() when pushBack != null:
return pushBack(_that);case EventDeltaEntity_Insert() when insert != null:
return insert(_that);case EventDeltaEntity_Remove() when remove != null:
return remove(_that);case EventDeltaEntity_Update() when update != null:
return update(_that);case EventDeltaEntity_Reset() when reset != null:
return reset(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( EventDeltaEntity_PushFront value)  pushFront,required TResult Function( EventDeltaEntity_PushBack value)  pushBack,required TResult Function( EventDeltaEntity_Insert value)  insert,required TResult Function( EventDeltaEntity_Remove value)  remove,required TResult Function( EventDeltaEntity_Update value)  update,required TResult Function( EventDeltaEntity_Reset value)  reset,}){
final _that = this;
switch (_that) {
case EventDeltaEntity_PushFront():
return pushFront(_that);case EventDeltaEntity_PushBack():
return pushBack(_that);case EventDeltaEntity_Insert():
return insert(_that);case EventDeltaEntity_Remove():
return remove(_that);case EventDeltaEntity_Update():
return update(_that);case EventDeltaEntity_Reset():
return reset(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( EventDeltaEntity_PushFront value)?  pushFront,TResult? Function( EventDeltaEntity_PushBack value)?  pushBack,TResult? Function( EventDeltaEntity_Insert value)?  insert,TResult? Function( EventDeltaEntity_Remove value)?  remove,TResult? Function( EventDeltaEntity_Update value)?  update,TResult? Function( EventDeltaEntity_Reset value)?  reset,}){
final _that = this;
switch (_that) {
case EventDeltaEntity_PushFront() when pushFront != null:
return pushFront(_that);case EventDeltaEntity_PushBack() when pushBack != null:
return pushBack(_that);case EventDeltaEntity_Insert() when insert != null:
return insert(_that);case EventDeltaEntity_Remove() when remove != null:
return remove(_that);case EventDeltaEntity_Update() when update != null:
return update(_that);case EventDeltaEntity_Reset() when reset != null:
return reset(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( EventEntity value)?  pushFront,TResult Function( EventEntity value)?  pushBack,TResult Function( int index,  EventEntity value)?  insert,TResult Function( int index)?  remove,TResult Function( int index,  EventEntity value)?  update,TResult Function( List<EventEntity> items)?  reset,required TResult orElse(),}) {final _that = this;
switch (_that) {
case EventDeltaEntity_PushFront() when pushFront != null:
return pushFront(_that.value);case EventDeltaEntity_PushBack() when pushBack != null:
return pushBack(_that.value);case EventDeltaEntity_Insert() when insert != null:
return insert(_that.index,_that.value);case EventDeltaEntity_Remove() when remove != null:
return remove(_that.index);case EventDeltaEntity_Update() when update != null:
return update(_that.index,_that.value);case EventDeltaEntity_Reset() when reset != null:
return reset(_that.items);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( EventEntity value)  pushFront,required TResult Function( EventEntity value)  pushBack,required TResult Function( int index,  EventEntity value)  insert,required TResult Function( int index)  remove,required TResult Function( int index,  EventEntity value)  update,required TResult Function( List<EventEntity> items)  reset,}) {final _that = this;
switch (_that) {
case EventDeltaEntity_PushFront():
return pushFront(_that.value);case EventDeltaEntity_PushBack():
return pushBack(_that.value);case EventDeltaEntity_Insert():
return insert(_that.index,_that.value);case EventDeltaEntity_Remove():
return remove(_that.index);case EventDeltaEntity_Update():
return update(_that.index,_that.value);case EventDeltaEntity_Reset():
return reset(_that.items);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( EventEntity value)?  pushFront,TResult? Function( EventEntity value)?  pushBack,TResult? Function( int index,  EventEntity value)?  insert,TResult? Function( int index)?  remove,TResult? Function( int index,  EventEntity value)?  update,TResult? Function( List<EventEntity> items)?  reset,}) {final _that = this;
switch (_that) {
case EventDeltaEntity_PushFront() when pushFront != null:
return pushFront(_that.value);case EventDeltaEntity_PushBack() when pushBack != null:
return pushBack(_that.value);case EventDeltaEntity_Insert() when insert != null:
return insert(_that.index,_that.value);case EventDeltaEntity_Remove() when remove != null:
return remove(_that.index);case EventDeltaEntity_Update() when update != null:
return update(_that.index,_that.value);case EventDeltaEntity_Reset() when reset != null:
return reset(_that.items);case _:
  return null;

}
}

}

/// @nodoc


class EventDeltaEntity_PushFront extends EventDeltaEntity {
  const EventDeltaEntity_PushFront({required this.value}): super._();
  

 final  EventEntity value;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EventDeltaEntity_PushFrontCopyWith<EventDeltaEntity_PushFront> get copyWith => _$EventDeltaEntity_PushFrontCopyWithImpl<EventDeltaEntity_PushFront>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity_PushFront&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,value);

@override
String toString() {
  return 'EventDeltaEntity.pushFront(value: $value)';
}


}

/// @nodoc
abstract mixin class $EventDeltaEntity_PushFrontCopyWith<$Res> implements $EventDeltaEntityCopyWith<$Res> {
  factory $EventDeltaEntity_PushFrontCopyWith(EventDeltaEntity_PushFront value, $Res Function(EventDeltaEntity_PushFront) _then) = _$EventDeltaEntity_PushFrontCopyWithImpl;
@useResult
$Res call({
 EventEntity value
});




}
/// @nodoc
class _$EventDeltaEntity_PushFrontCopyWithImpl<$Res>
    implements $EventDeltaEntity_PushFrontCopyWith<$Res> {
  _$EventDeltaEntity_PushFrontCopyWithImpl(this._self, this._then);

  final EventDeltaEntity_PushFront _self;
  final $Res Function(EventDeltaEntity_PushFront) _then;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? value = null,}) {
  return _then(EventDeltaEntity_PushFront(
value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as EventEntity,
  ));
}


}

/// @nodoc


class EventDeltaEntity_PushBack extends EventDeltaEntity {
  const EventDeltaEntity_PushBack({required this.value}): super._();
  

 final  EventEntity value;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EventDeltaEntity_PushBackCopyWith<EventDeltaEntity_PushBack> get copyWith => _$EventDeltaEntity_PushBackCopyWithImpl<EventDeltaEntity_PushBack>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity_PushBack&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,value);

@override
String toString() {
  return 'EventDeltaEntity.pushBack(value: $value)';
}


}

/// @nodoc
abstract mixin class $EventDeltaEntity_PushBackCopyWith<$Res> implements $EventDeltaEntityCopyWith<$Res> {
  factory $EventDeltaEntity_PushBackCopyWith(EventDeltaEntity_PushBack value, $Res Function(EventDeltaEntity_PushBack) _then) = _$EventDeltaEntity_PushBackCopyWithImpl;
@useResult
$Res call({
 EventEntity value
});




}
/// @nodoc
class _$EventDeltaEntity_PushBackCopyWithImpl<$Res>
    implements $EventDeltaEntity_PushBackCopyWith<$Res> {
  _$EventDeltaEntity_PushBackCopyWithImpl(this._self, this._then);

  final EventDeltaEntity_PushBack _self;
  final $Res Function(EventDeltaEntity_PushBack) _then;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? value = null,}) {
  return _then(EventDeltaEntity_PushBack(
value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as EventEntity,
  ));
}


}

/// @nodoc


class EventDeltaEntity_Insert extends EventDeltaEntity {
  const EventDeltaEntity_Insert({required this.index, required this.value}): super._();
  

 final  int index;
 final  EventEntity value;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EventDeltaEntity_InsertCopyWith<EventDeltaEntity_Insert> get copyWith => _$EventDeltaEntity_InsertCopyWithImpl<EventDeltaEntity_Insert>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity_Insert&&(identical(other.index, index) || other.index == index)&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,index,value);

@override
String toString() {
  return 'EventDeltaEntity.insert(index: $index, value: $value)';
}


}

/// @nodoc
abstract mixin class $EventDeltaEntity_InsertCopyWith<$Res> implements $EventDeltaEntityCopyWith<$Res> {
  factory $EventDeltaEntity_InsertCopyWith(EventDeltaEntity_Insert value, $Res Function(EventDeltaEntity_Insert) _then) = _$EventDeltaEntity_InsertCopyWithImpl;
@useResult
$Res call({
 int index, EventEntity value
});




}
/// @nodoc
class _$EventDeltaEntity_InsertCopyWithImpl<$Res>
    implements $EventDeltaEntity_InsertCopyWith<$Res> {
  _$EventDeltaEntity_InsertCopyWithImpl(this._self, this._then);

  final EventDeltaEntity_Insert _self;
  final $Res Function(EventDeltaEntity_Insert) _then;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? index = null,Object? value = null,}) {
  return _then(EventDeltaEntity_Insert(
index: null == index ? _self.index : index // ignore: cast_nullable_to_non_nullable
as int,value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as EventEntity,
  ));
}


}

/// @nodoc


class EventDeltaEntity_Remove extends EventDeltaEntity {
  const EventDeltaEntity_Remove({required this.index}): super._();
  

 final  int index;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EventDeltaEntity_RemoveCopyWith<EventDeltaEntity_Remove> get copyWith => _$EventDeltaEntity_RemoveCopyWithImpl<EventDeltaEntity_Remove>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity_Remove&&(identical(other.index, index) || other.index == index));
}


@override
int get hashCode => Object.hash(runtimeType,index);

@override
String toString() {
  return 'EventDeltaEntity.remove(index: $index)';
}


}

/// @nodoc
abstract mixin class $EventDeltaEntity_RemoveCopyWith<$Res> implements $EventDeltaEntityCopyWith<$Res> {
  factory $EventDeltaEntity_RemoveCopyWith(EventDeltaEntity_Remove value, $Res Function(EventDeltaEntity_Remove) _then) = _$EventDeltaEntity_RemoveCopyWithImpl;
@useResult
$Res call({
 int index
});




}
/// @nodoc
class _$EventDeltaEntity_RemoveCopyWithImpl<$Res>
    implements $EventDeltaEntity_RemoveCopyWith<$Res> {
  _$EventDeltaEntity_RemoveCopyWithImpl(this._self, this._then);

  final EventDeltaEntity_Remove _self;
  final $Res Function(EventDeltaEntity_Remove) _then;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? index = null,}) {
  return _then(EventDeltaEntity_Remove(
index: null == index ? _self.index : index // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class EventDeltaEntity_Update extends EventDeltaEntity {
  const EventDeltaEntity_Update({required this.index, required this.value}): super._();
  

 final  int index;
 final  EventEntity value;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EventDeltaEntity_UpdateCopyWith<EventDeltaEntity_Update> get copyWith => _$EventDeltaEntity_UpdateCopyWithImpl<EventDeltaEntity_Update>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity_Update&&(identical(other.index, index) || other.index == index)&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,index,value);

@override
String toString() {
  return 'EventDeltaEntity.update(index: $index, value: $value)';
}


}

/// @nodoc
abstract mixin class $EventDeltaEntity_UpdateCopyWith<$Res> implements $EventDeltaEntityCopyWith<$Res> {
  factory $EventDeltaEntity_UpdateCopyWith(EventDeltaEntity_Update value, $Res Function(EventDeltaEntity_Update) _then) = _$EventDeltaEntity_UpdateCopyWithImpl;
@useResult
$Res call({
 int index, EventEntity value
});




}
/// @nodoc
class _$EventDeltaEntity_UpdateCopyWithImpl<$Res>
    implements $EventDeltaEntity_UpdateCopyWith<$Res> {
  _$EventDeltaEntity_UpdateCopyWithImpl(this._self, this._then);

  final EventDeltaEntity_Update _self;
  final $Res Function(EventDeltaEntity_Update) _then;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? index = null,Object? value = null,}) {
  return _then(EventDeltaEntity_Update(
index: null == index ? _self.index : index // ignore: cast_nullable_to_non_nullable
as int,value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as EventEntity,
  ));
}


}

/// @nodoc


class EventDeltaEntity_Reset extends EventDeltaEntity {
  const EventDeltaEntity_Reset({required final  List<EventEntity> items}): _items = items,super._();
  

 final  List<EventEntity> _items;
 List<EventEntity> get items {
  if (_items is EqualUnmodifiableListView) return _items;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_items);
}


/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EventDeltaEntity_ResetCopyWith<EventDeltaEntity_Reset> get copyWith => _$EventDeltaEntity_ResetCopyWithImpl<EventDeltaEntity_Reset>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EventDeltaEntity_Reset&&const DeepCollectionEquality().equals(other._items, _items));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_items));

@override
String toString() {
  return 'EventDeltaEntity.reset(items: $items)';
}


}

/// @nodoc
abstract mixin class $EventDeltaEntity_ResetCopyWith<$Res> implements $EventDeltaEntityCopyWith<$Res> {
  factory $EventDeltaEntity_ResetCopyWith(EventDeltaEntity_Reset value, $Res Function(EventDeltaEntity_Reset) _then) = _$EventDeltaEntity_ResetCopyWithImpl;
@useResult
$Res call({
 List<EventEntity> items
});




}
/// @nodoc
class _$EventDeltaEntity_ResetCopyWithImpl<$Res>
    implements $EventDeltaEntity_ResetCopyWith<$Res> {
  _$EventDeltaEntity_ResetCopyWithImpl(this._self, this._then);

  final EventDeltaEntity_Reset _self;
  final $Res Function(EventDeltaEntity_Reset) _then;

/// Create a copy of EventDeltaEntity
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? items = null,}) {
  return _then(EventDeltaEntity_Reset(
items: null == items ? _self._items : items // ignore: cast_nullable_to_non_nullable
as List<EventEntity>,
  ));
}


}

// dart format on
