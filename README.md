# Conversion Output

Macro has simple cache implementation for repeating function calls, so it doesn't lookup everytime you call..


```cpp
  v2 = *(_QWORD *)(v47 + 8);
    if ( !v2 )
    {
      v39 = v48;
      v44 = v47;
      Ldr = NtCurrentPeb()->Ldr;
      Blink = Ldr->InLoadOrderModuleList.Blink;
      Flink = Ldr->InLoadOrderModuleList.Flink;
      if ( Flink == Blink )
        goto LABEL_54;
      v5 = 36018504;
      v40 = 0;
      v6 = Ldr->InLoadOrderModuleList.Flink;
      v7 = 0;
      v8 = 0;
      v42 = Blink;
      while ( 1 )
      {
        v9 = (unsigned __int8 *)v6[6].Flink;
        v10 = &v9[(LODWORD(v6[5].Blink) - 8) & 0xFFFE];
        v11 = 2507023;
        if ( v9 < v10 )
        {
          v12 = (unsigned __int8 *)v6[6].Flink;
          do
          {
            v13 = *v12 | 0x20;
            if ( (unsigned __int8)(*v12 - 65) >= 0x1Au )
              v13 = *v12;
            v11 = -1369772439 * (v11 ^ v13);
            v12 += 2;
          }
          while ( v12 < v10 );
        }
        if ( v8 && v11 != v8 && (!v11 || v11 == v7) )
          goto LABEL_49;
        v14 = v6[3].Flink;
        Blink_high = SHIDWORD(v14[3].Blink);
        v16 = *(unsigned int *)((char *)&v14[8].Blink + Blink_high);
        if ( !*(_DWORD *)((char *)&v14[8].Blink + Blink_high) )
          goto LABEL_49;
        v45 = SHIDWORD(v14[3].Blink);
        v17 = (_DWORD *)((char *)v14 + v16);
        v18 = 0i64;
        do
        {
          if ( v18 == v17[6] )
          {
            Blink = v42;
            goto LABEL_49;
          }
          v19 = v18;
          v0 = (char *)v14 + *(unsigned int *)((char *)&v14->Flink + 4 * v18 + (unsigned int)v17[8]);
          v20 = -1i64;
          v21 = 0i64;
          do
          {
            v22 = v21;
            v23 = v0[++v20] == 0;
            ++v21;
          }
          while ( !v23 );
          v24 = 2507023;
          if ( v20 )
          {
            do
            {
              v25 = *v0;
              if ( !*v0 )
                break;
              ++v0;
              v26 = v25 - 65;
              v27 = v25;
              v28 = v25 | 0x20;
              if ( v26 >= 0x1Au )
                v28 = v27;
              v24 = -1369772439 * (v28 ^ v24);
              --v22;
            }
            while ( v22 );
          }
          v18 = v19 + 1;
        }
        while ( v24 != v5 );
        if ( (v40 & 1) == 0 )
        {
          v7 = 2507023;
          if ( v9 < v10 )
          {
            v7 = 2507023;
            do
            {
              v0 = (char *)*v9;
              v29 = (unsigned __int8)v0 | 0x20;
              if ( (unsigned __int8)(*v9 - 65) >= 0x1Au )
                v29 = *v9;
              v7 = -1369772439 * (v7 ^ v29);
              v9 += 2;
            }
            while ( v9 < v10 );
          }
        }
        v30 = (unsigned __int64)v14
            + *(unsigned int *)((char *)&v14->Flink
                              + 4 * *(unsigned __int16 *)((char *)&v14->Flink + 2 * v19 + (unsigned int)v17[9])
                              + (unsigned int)v17[7]);
        if ( (unsigned __int64)v17 >= v30 )
          break;
        Blink = v42;
        if ( (unsigned __int64)v17 + *(unsigned int *)((char *)&v14[8].Blink + v45 + 4) <= v30 )
          goto LABEL_52;
        v31 = (unsigned __int8 *)(v30 + 2);
        v8 = 2507023;
        while ( 1 )
        {
          v32 = *(unsigned __int8 *)v30;
          if ( !*(_BYTE *)v30 )
          {
            v40 = 1;
LABEL_47:
            v5 = 2507023;
            goto LABEL_48;
          }
          if ( v32 == 46 )
            break;
          v33 = v32 - 65;
          v34 = v32 | 0x20;
          if ( v33 >= 0x1Au )
            v34 = *(_BYTE *)v30;
          v8 = -1369772439 * (v34 ^ v8);
          ++v30;
          ++v31;
        }
        v35 = *(_BYTE *)(v30 + 1);
        v40 = 1;
        if ( !v35 )
          goto LABEL_47;
        v5 = 2507023;
        do
        {
          v36 = v35 - 65;
          v37 = v35;
          v38 = v35 | 0x20;
          if ( v36 >= 0x1Au )
            v38 = v37;
          v5 = -1369772439 * (v5 ^ v38);
          v35 = *v31++;
        }
        while ( v35 );
LABEL_48:
        v6 = Flink;
LABEL_49:
        v6 = v6->Flink;
        if ( v6 == Blink )
        {
LABEL_54:
          v46 = (__int64)&off_140029610;
          v47 = 1i64;
          v48 = 8i64;
          v49 = 0i64;
          sub_140026FE0(&v46, &off_140029650);
        }
      }
      if ( !v30 )
        goto LABEL_54;
LABEL_52:
      v2 = ~__ROL8__(v30 ^ 0x889AEC13A7ADD3AAui64, 24);
      v1 = v44;
      *(_QWORD *)(v44 + 8) = v2;
      LOBYTE(v0) = v39;
    }
    sub_1400016D5(v1, v0);
    ((void (__fastcall *)(__int64))(__ROL8__(~v2, 40) ^ 0x889AEC13A7ADD3AAui64))(1001i64);
  }
}
```

# License

[LICENSE - Apache 2.0](./LICENSE)

# Credit
Apache 2.0 - [kkent030315/razy_importer](https://github.com/kkent030315/razy_importer)
Apache 2.0 - [JustasMasiulis/lazy_importer](https://github.com/JustasMasiulis/lazy_importer)