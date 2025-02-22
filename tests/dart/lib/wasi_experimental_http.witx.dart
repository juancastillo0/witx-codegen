
//
// This file was automatically generated by witx-codegen - Do not edit manually.
//

// ignore_for_file: non_constant_identifier_names

import 'dart:convert' show utf8;
import 'dart:ffi';
import 'dart:typed_data';

import 'package:ffi/ffi.dart' show malloc;

// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum Error {
//     WasiError(i32),
// }
// impl std::error::Error for Error {}
// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::WasiError(e) => write!(f, "Wasi error {}", e),
//         }
//     }
// }

class WasiError {
    final int code;
    const WasiError(this.code);

    @override
    String toString() {
        return 'WasiError{code: $code}';
    }
}

sealed class Result<O, E> {}

class Ok<O> implements Result<O, Never> {
    final O ok;
    const Ok(this.ok);
}

class Err<E> implements Result<Never, E> {
    final E err;
    const Err(this.err);
}

typedef i8 = Int8;
typedef i16 = Int16;
typedef i32 = Int32;
typedef i64 = Int64;
typedef u8 = Uint8;
typedef u16 = Uint16;
typedef u32 = Uint32;
typedef u64 = Uint64;
typedef usize = UintPtr;

typedef WasiHandle = i32;
typedef Char8 = u8;
typedef Char32 = u32;
typedef WasiPtr<T extends NativeType> = Pointer<T>;
typedef WasiMutPtr<T extends NativeType> = Pointer<T>;
typedef WasiStringBytesPtr = WasiPtr<Char8>;

// typedef WasiSlice<T extends NativeType> = Array<T>;

final class WasiSlice extends Struct {
    external WasiPtr<NativeType> ptr;
    @usize()
    external int len;

    List<T> asSlice<T>({
        required int itemSize,
        required T Function(WasiPtr<NativeType> ptr) fromPointer,
    }) {
        final result = <T>[];
        for (var i = ptr.address; i < len * itemSize; i += itemSize) {
            result.add(fromPointer(Pointer.fromAddress(i)));
        }
        return result;
    }

    static WasiSlice fromSlice<T>(
        List<T> slice, {
        required int itemSize,
        required void Function(T value, WasiPtr<NativeType> ptr) populatePointer,
    }) {
        final result = malloc<WasiSlice>();
        result.ref.ptr = malloc.allocate(slice.length * itemSize);
        result.ref.len = slice.length;
        final address = result.ref.ptr.address;
        for (var i = 0; i < slice.length; i++) {
            populatePointer(
                slice[i],
                Pointer.fromAddress(address + i * itemSize),
            );
        }
        return result.ref;
    }
}

final class WasiString extends Struct {
    external WasiStringBytesPtr ptr;
    @usize()
    external int len;

    static WasiString fromUtf8Slice(List<int> slice) {
        final stringPointer = malloc<WasiString>();
        final Pointer<Uint8> result = malloc<Uint8>(slice.length);
        result.asTypedList(slice.length).setAll(0, slice);

        return stringPointer.ref
            ..ptr = result
            ..len = slice.length;
    }

    static WasiString fromString(String v) {
        return fromUtf8Slice(utf8.encode(v));
    }

    Uint8List toUtf8Slice() {
        return ptr.asTypedList(len);
    }

    @override
    String toString() {
        final c = ptr.asTypedList(len);
        return utf8.decode(c);
    }
}

// ---------------------- Module: [wasi_experimental_http] ----------------------

typedef HttpError = int /* u32 */;

class HTTP_ERROR {
    static const HttpError SUCCESS = 0;
    static const HttpError INVALID_HANDLE = 1;
    static const HttpError MEMORY_NOT_FOUND = 2;
    static const HttpError MEMORY_ACCESS_ERROR = 3;
    static const HttpError BUFFER_TOO_SMALL = 4;
    static const HttpError HEADER_NOT_FOUND = 5;
    static const HttpError UTF_8_ERROR = 6;
    static const HttpError DESTINATION_NOT_ALLOWED = 7;
    static const HttpError INVALID_METHOD = 8;
    static const HttpError INVALID_ENCODING = 9;
    static const HttpError INVALID_URL = 10;
    static const HttpError REQUEST_ERROR = 11;
    static const HttpError RUNTIME_ERROR = 12;
    static const HttpError TOO_MANY_SESSIONS = 13;
}

/// HTTP status code
typedef StatusCode = int /* u16 */;

/// An HTTP body being sent
typedef OutgoingBody = WasiSlice /* <int /* u8 */> */;

/// Buffer for an HTTP body being received
typedef IncomingBody = WasiSlice /* Mut <int /* u8 */> */;

/// A response handle
typedef ResponseHandle = WasiHandle;

/// Buffer to store a header value
typedef HeaderValueBuf = WasiSlice /* Mut <int /* u8 */> */;

/// Number of bytes having been written
typedef WrittenBytes = int /* usize */;

/// Send a request
Result<(StatusCode, ResponseHandle), Error> req(
    WasiPtr<u8> urlPtr,
    int /* usize */ urlLen,
    WasiPtr<u8> methodPtr,
    int /* usize */ methodLen,
    WasiPtr<u8> headersPtr,
    int /* usize */ headersLen,
    WasiPtr<u8> bodyPtr,
    int /* usize */ bodyLen,
) {
    final req = DynamicLibrary.open('wasi_experimental_http').lookupFunction<
        int /* u32 */ /* Enum */ Function(

            int /* Char8 */ urlPtr,
            usize urlLen,
            int /* Char8 */ methodPtr,
            usize methodLen,
            int /* Char8 */ headersPtr,
            usize headersLen,
            int /* u8 */ bodyPtr,
            usize bodyLen,
            StatusCode result0Ptr,
            ResponseHandle result1Ptr,
        ),            HttpError Function(

                WasiPtr<u8> urlPtr,
                int /* usize */ urlLen,
                WasiPtr<u8> methodPtr,
                int /* usize */ methodLen,
                WasiPtr<u8> headersPtr,
                int /* usize */ headersLen,
                WasiPtr<u8> bodyPtr,
                int /* usize */ bodyLen,
                WasiPtr<u16> /* Mut */ result0Ptr,
                WasiPtr<WasiHandle> /* Mut */ result1Ptr,
            )>('req');    final result0Ptr = malloc<u16>();
    final result1Ptr = malloc<WasiHandle>();
    final res = req(
        urlPtr,
        urlLen,
        methodPtr,
        methodLen,
        headersPtr,
        headersLen,
        bodyPtr,
        bodyLen,
        result0Ptr,
        result1Ptr,
    );
    if (res != 0) {
        return Err(WasiError(res));
    }
    return Ok((result0Ptr.ref, result1Ptr.ref));
}

/// Close a request handle
Result<(), Error> close(
    ResponseHandle responseHandle,
) {
    final close = DynamicLibrary.open('wasi_experimental_http').lookupFunction<
        int /* u32 */ /* Enum */ Function(

            WasiHandle responseHandle,
        ),            HttpError Function(

                ResponseHandle responseHandle,
            )>('close');    final res = close(
        responseHandle,
    );
    if (res != 0) {
        return Err(WasiError(res));
    }
    return Ok(());
}

/// Get the value associated with a header
Result<WrittenBytes, Error> headerGet(
    ResponseHandle responseHandle,
    WasiPtr<u8> headerNamePtr,
    int /* usize */ headerNameLen,
    WasiPtr<u8> /* Mut */ headerValueBufPtr,
    int /* usize */ headerValueBufLen,
) {
    final headerGet = DynamicLibrary.open('wasi_experimental_http').lookupFunction<
        int /* u32 */ /* Enum */ Function(

            WasiHandle responseHandle,
            int /* Char8 */ headerNamePtr,
            usize headerNameLen,
            int /* u8 */ headerValueBufPtr,
            usize headerValueBufLen,
            WrittenBytes resultPtr,
        ),            HttpError Function(

                ResponseHandle responseHandle,
                WasiPtr<u8> headerNamePtr,
                int /* usize */ headerNameLen,
                WasiPtr<u8> /* Mut */ headerValueBufPtr,
                int /* usize */ headerValueBufLen,
                WasiPtr<usize> /* Mut */ resultPtr,
            )>('header_get');    final resultPtr = malloc<usize>();
    final res = headerGet(
        responseHandle,
        headerNamePtr,
        headerNameLen,
        headerValueBufPtr,
        headerValueBufLen,
        resultPtr,
    );
    if (res != 0) {
        return Err(WasiError(res));
    }
    return Ok(resultPtr.ref);
}

/// Fill a buffer with the streamed content of a response body
Result<WrittenBytes, Error> bodyRead(
    ResponseHandle responseHandle,
    WasiPtr<u8> /* Mut */ bodyBufPtr,
    int /* usize */ bodyBufLen,
) {
    final bodyRead = DynamicLibrary.open('wasi_experimental_http').lookupFunction<
        int /* u32 */ /* Enum */ Function(

            WasiHandle responseHandle,
            int /* u8 */ bodyBufPtr,
            usize bodyBufLen,
            WrittenBytes resultPtr,
        ),            HttpError Function(

                ResponseHandle responseHandle,
                WasiPtr<u8> /* Mut */ bodyBufPtr,
                int /* usize */ bodyBufLen,
                WasiPtr<usize> /* Mut */ resultPtr,
            )>('body_read');    final resultPtr = malloc<usize>();
    final res = bodyRead(
        responseHandle,
        bodyBufPtr,
        bodyBufLen,
        resultPtr,
    );
    if (res != 0) {
        return Err(WasiError(res));
    }
    return Ok(resultPtr.ref);
}

