# random number generator


```
mov  ax, dx   ; move dx to ax
xor  dx, dx   ; clear dx
mov  cx, 10   ; move decimal 10 to cx
div  cx       ; here dx contains the remainder of the division - from 0 to 9
add  dl, '0'  ; to ascii from '0' to '9'
```

1. We moved value in DX to AX 
2. We cleared DX. 
3. We moved 10 dec to CX. 
4. We divided AX by CX hence we get a remainder within 0-9 Dec which is stored in DX 
5. Finally, we added ASCII '0' (dec 48) to DX to get them into ASCII '0' to '9'.