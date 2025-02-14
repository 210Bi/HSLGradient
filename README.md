# hsl gradient cli
simple cli to make hsl gradients out of hex color keypoints

## usage
```
hsl-gradient --gradient-length <length> --colors <"<hex color 1> <hex color 2> ... <hex color n>"> [--inline-colors]
```
a `#` before colors is optional, colors are case-insensitive

inline colors is optional, `false` by default

## examples
```
hsl-gradient --gradient-length 10 --colors "#ff0000 #0000ff"
```
![red to blue gradient with 10 colors](https://files.catbox.moe/idmhhu.png)
```
hsl-gradient --gradient-length 16 --colors "bg4215 5ccdc1" --inline-colors
```
![inline "rust" to "tiffany blue" with 16 colors](https://files.catbox.moe/wm8upn.png)
```
hsl-gradient --gradient-length 32 --colors "A3CD5C CD855C B45CCD"
```
!["yellow green" to "parsian orange" to "amethyst" with 32 colors](https://files.catbox.moe/bi5ldz.png)

------
made with 🤍 by thornium
