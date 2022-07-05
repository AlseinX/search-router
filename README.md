# search-router

## Usage

+ Open your browser settings (Tested with Microsoft Edge)

+ Add a new search engine configuration:

```
http://sr.ovyno.com/?q=%s
```
+ Search in the url box. If there are Chinese words, it will redirect to `baidu.com`, otherwise `google.com`.

## Note

This checks if the query contains any Chinese word by unicode range `0x4e00 ..= 0x9fa5`, therefore Japanese words containing Kanji would also be recognized as Chinese and redirected to `baidu.com`. This is unavoidable because CJK languages share Chinese charactors in Unicode, and it would cost too much to detect language by AI solutions.

You could also use prefix to specify a searching engine manually, eg.:

+ `g 初めまして` goes to `google.com` searching `初めまして`

+ `b hello world` goes to `baidu.com` searching `hello world`

### The full prefix list

| Prefix | Engine |
| - | - |
| `g` or `google` | `google.com` |
| `b` or `baidu` | `baidu.com` |
| `bd` | `cn.bing.com` demestic |
| `bi` or `bing` | `cn.bing.com` international |

## License

This repository is open-sourced with [MIT License](./LICENSE).