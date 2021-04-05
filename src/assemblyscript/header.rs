use super::*;
use std::io::Write;

impl AssemblyScriptGenerator {
    pub fn header<T: Write>(w: &mut PrettyWriter<T>) -> Result<(), Error> {
        w.write_lines(
            "
/*
 * This file was automatically generated by witx-codegen - Do not edit manually.
 */",
        )?;
        w.write_lines(
            "
export type WasiHandle = i32;
export type Char8 = u8;
export type Char32 = u32;
export type WasiPtr<T> = usize;
export type WasiMutPtr<T> = usize;
export type WasiStringBytesPtr = WasiMutPtr<Char8>;
",
        )?;
        w.write_lines(
            "
@unmanaged
export class WasiString {
    ptr: WasiStringBytesPtr;
    length: usize;

    constructor(str: string) {
        let wasiString = String.UTF8.encode(str, false);
        // @ts-ignore: cast
        this.ptr = changetype<WasiStringBytesPtr>(wasiString);
        this.length = wasiString.byteLength;
    }

    toString(): string {
        let tmp = new ArrayBuffer(this.length as u32);
        memory.copy(changetype<usize>(tmp), this.ptr, this.length);
        return String.UTF8.decode(tmp);
    }
}

@unmanaged
export class WasiSlice<T> {
    ptr: WasiPtr<T>;
    length: usize;

    constructor(array: ArrayBufferView) {
        // @ts-ignore: cast
        this.ptr = array.dataStart;
        this.length = array.byteLength;
    }
}

@unmanaged
export class WasiMutSlice<T> {
    ptr: WasiMutPtr<T>;
    length: usize;

    constructor(array: ArrayBufferView) {
        // @ts-ignore: cast
        this.ptr = array.dataStart;
        this.length = array.byteLength;
    }
}
",
        )?
        .eob()?;
        Ok(())
    }
}
