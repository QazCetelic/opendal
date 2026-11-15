## Capabilities

There are many WebDAV server implementations, but they are unfortunately not all to standard.
The table below shows these differences.
If undefined behavior from deviating implementations is not acceptable, consider disabling capabilities with the `CapabilityOverrideLayer`.
Also note that some servers derive ETags from the modification time with one-second resolution, this means the preconditions won't work reliably if requests are sent within that window.

| Capability (mechanism) | Apache httpd | lighttpd | SabreDAV (Nextcloud, ownCloud) | nginx |
| --- | --- | --- | --- | --- |
| `stat`, `list` (`PROPFIND`) | Yes | Yes | Yes | Yes |
| `read` (`GET`) | Yes | Yes | Yes | Yes |
| `read_with_suffix` (`Range` on `GET`) | Yes | Yes | Yes | Yes |
| `read_with_if_match` (`If-Match` on `GET`) | No (weak ETag) | No (GET unchecked) | Yes | No (ignored) |
| `read_with_if_none_match` (`If-None-Match` on `GET`) | Yes | No (GET unchecked) | Yes | No (ignored) |
| `read_with_if_modified_since` (`If-Modified-Since` on `GET`) | Yes | No (GET unchecked) | Yes | No (ignored) |
| `read_with_if_unmodified_since` (`If-Unmodified-Since` on `GET`) | Yes | No (GET unchecked) | Yes | No (ignored) |
| `write`, `write_can_empty` (`PUT`) | Yes | Yes | Yes | Yes |
| `write_with_if_match` (tagged `If` list on `PUT`) | Yes | Yes | Yes | No (ignored) |
| `write_with_if_none_match` (`If-None-Match` and tagged `If` list on `PUT`) | Yes | Yes | Yes | No (ignored) |
| `write_with_if_not_exists` (`If-None-Match: *` on `PUT`) | Yes | Yes | Yes | No (ignored) |
| `write_with_user_metadata` (`PROPPATCH`) | No (not implemented) | Yes | No (test disabled) | No (not implemented) |
| `create_dir` (`MKCOL`) | Yes | Yes | Yes | Yes |
| `delete` (`DELETE`) | Yes | Yes | Yes | Yes |
| `delete_with_if_match` (tagged `If` list on `DELETE`) | Yes | Yes | Yes | No (ignored) |
| `delete_with_if_none_match` (`If-None-Match` and tagged `If` list on `DELETE`) | Yes | Yes | Yes | No (ignored) |
| `copy` (`COPY`) | Yes | Yes | Yes | Yes |
| `copy_with_if_not_exists` (`Overwrite: F` on `COPY`) | Yes | Yes | Yes | Yes |
| `copy_with_if_match` (tagged `If` list on the `COPY` destination) | Yes | No (source checked) | Yes | No (ignored) |
| `rename` (`MOVE`) | Yes | Yes | Yes | Yes |
| `rename_with_if_not_exists` (`Overwrite: F` on `MOVE`) | Yes | Yes | Yes | Yes |
| `shared` (server-side state) | Yes | Yes | Yes | Yes |

## Notes

Bazel Remote Caching and Ccache HTTP Storage is also part of this service.
Users can use `webdav` to connect those services.

ETag conditions travel in a tagged `If` list naming the resource they describe:
the request URI for `read`, `write` and `delete`, and the destination for
`copy`. `if_none_match` also sends `If-None-Match`, because implementations
enforce different one of the two spellings. `if_not_exists` sends
`If-None-Match: *` on `PUT` and `Overwrite: F` on `COPY` and `MOVE`. A negated
ETag condition is not available for `copy`, because no tested server enforces
`Not [...]` on the destination.

## Configuration

Use [`crate::WebdavConfig`] for serializable configuration and this builder's
methods for direct construction. The field and method documentation defines
accepted values, defaults, and environment interaction.

## Example

### Via Builder

```rust,no_run
use anyhow::Result;
use opendal_service_webdav::Webdav;
use opendal_core::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Webdav::default()
        .endpoint("127.0.0.1")
        .username("xxx")
        .password("xxx");

    let op: Operator = Operator::new(builder)?;

    Ok(())
}
```
