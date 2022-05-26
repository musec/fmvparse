This library is used for parsing `.mp4` file formats and extract the internal structures.

### Usage
```sh
cargo run --release -- -f <some mp4-file>
```

Dump all metadata found in the source file.

```
ftyp: {size: 28, start address: 0, end address: 28}
wide: {size: 8, start address: 28, end address: 36}
mdat: {size: 457745015, start address: 36, end address: 457745051}
moov: {size: 30854, start address: 457745051, end address: 457775905}
        mvhd: {size: 108, start address: 457745059, end address: 457745167}
        udta: {size: 2048, start address: 457745167, end address: 457747215}
        trak: {size: 26112, start address: 457747215, end address: 457773327}
                tkhd: {size: 92, start address: 457747223, end address: 457747315}
                edts: {size: 36, start address: 457747315, end address: 457747351}
                mdia: {size: 25976, start address: 457747351, end address: 457773327}
                        mdhd: {size: 32, start address: 457747359, end address: 457747391}
                        hdlr: {size: 49, start address: 457747391, end address: 457747440}
                        minf: {size: 25887, start address: 457747440, end address: 457773327}
                                vmhd: {size: 20, start address: 457747448, end address: 457747468}
                                dinf: {size: 36, start address: 457747468, end address: 457747504}
                                stbl: {size: 25823, start address: 457747504, end address: 457773327}
                                        stsd: {size: 179, start address: 457747512, end address: 457747691}
                                        stts: {size: 24, start address: 457747691, end address: 457747715}
                                        ctts: {size: 24, start address: 457747715, end address: 457747739}
                                        stsc: {size: 28, start address: 457747739, end address: 457747767}
                                        stsz: {size: 12564, start address: 457747767, end address: 457760331}
                                        stco: {size: 12560, start address: 457760331, end address: 457772891}
                                                        offsets:{98067, 772564, ...}
                                        stss: {size: 436, start address: 457772891, end address: 457773327}
        trak: {size: 1265, start address: 457773327, end address: 457774592}
                tkhd: {size: 92, start address: 457773335, end address: 457773427}
                mdia: {size: 1165, start address: 457773427, end address: 457774592}
                        mdhd: {size: 32, start address: 457773435, end address: 457773467}
                        hdlr: {size: 49, start address: 457773467, end address: 457773516}
                        minf: {size: 1076, start address: 457773516, end address: 457774592}
                                gmhd: {size: 56, start address: 457773524, end address: 457773580}
                                dinf: {size: 36, start address: 457773580, end address: 457773616}
                                stbl: {size: 976, start address: 457773616, end address: 457774592}
                                        stsd: {size: 32, start address: 457773624, end address: 457773656}
                                        stts: {size: 24, start address: 457773656, end address: 457773680}
                                        stsc: {size: 28, start address: 457773680, end address: 457773708}
                                        stsz: {size: 444, start address: 457773708, end address: 457774152}
                                        stco: {size: 440, start address: 457774152, end address: 457774592}
                                                        offsets:{44, 97561, ...}
        trak: {size: 1313, start address: 457774592, end address: 457775905}
                tkhd: {size: 92, start address: 457774600, end address: 457774692}
                edts: {size: 36, start address: 457774692, end address: 457774728}
                mdia: {size: 1177, start address: 457774728, end address: 457775905}
                        mdhd: {size: 32, start address: 457774736, end address: 457774768}
                        hdlr: {size: 49, start address: 457774768, end address: 457774817}
                        minf: {size: 1088, start address: 457774817, end address: 457775905}
                                gmhd: {size: 76, start address: 457774825, end address: 457774901}
                                dinf: {size: 36, start address: 457774901, end address: 457774937}
                                stbl: {size: 968, start address: 457774937, end address: 457775905}
                                        stsd: {size: 32, start address: 457774945, end address: 457774977}
                                        stts: {size: 24, start address: 457774977, end address: 457775001}
                                        stsc: {size: 28, start address: 457775001, end address: 457775029}
                                        stsz: {size: 440, start address: 457775029, end address: 457775469}
                                        stco: {size: 436, start address: 457775469, end address: 457775905}
                                                        offsets:{97811, 4583912, ...}
```