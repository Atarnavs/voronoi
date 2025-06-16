package utils.http;

import java.io.IOException;
import java.text.ParseException;

import utils.gson.GsonUtil;
import org.apache.http.HttpEntity;
import org.apache.http.HttpHeaders;
import org.apache.http.client.methods.CloseableHttpResponse;
import org.apache.http.client.methods.HttpDelete;
import org.apache.http.client.methods.HttpGet;
import org.apache.http.client.methods.HttpPost;
import org.apache.http.client.methods.HttpPut;
import org.apache.http.entity.ContentType;
import org.apache.http.entity.StringEntity;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClientBuilder;
import org.apache.http.util.EntityUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

//import utils.gson.GsonUtil;

public class HttpUtil {
    private static final Logger log = LoggerFactory.getLogger(HttpUtil.class);

    /**
     * singleton HttpClientBuilder
     */
    private static HttpClientBuilder httpClientBuilder = HttpClientBuilder.create();

    /**
     * Use HTTPClient to perform a GET request of the given URI and return the response.
     *
     * @param uri the URI to GET
     * @return a Response containing the status code and body.
     * @throws ParseException
     * @throws IOException
     */
    public static RestResponse doGet(String uri) throws IOException {
        HttpGet method = new HttpGet(uri);
        method.addHeader(HttpHeaders.ACCEPT, ContentType.APPLICATION_JSON.getMimeType());
        /*
         * FIXME: not everything should reply with "application/json"
         * https://infrared5.atlassian.net/browse/AUT-227
         * https://infrared5.atlassian.net/browse/RPRO-8810
         * https://infrared5.atlassian.net/browse/RPRO-8856
         */

        log.debug("GET {}", uri);
        CloseableHttpClient httpclient = httpClientBuilder.build();
        CloseableHttpResponse response = httpclient.execute(method);
        HttpEntity responseEntity = response.getEntity();
        // Some things reply with 204 No Content.
        String responseBody = responseEntity != null ? EntityUtils.toString(responseEntity) : null;
        httpclient.close();

        log.debug(response.getStatusLine().toString());
        log.trace("\n" + responseBody);

        return new RestResponse(response.getStatusLine().getStatusCode(), responseBody);
    }

    /**
     * Use HTTPClient to perform an POST with the given jsonBody to the given URI and return the response.
     *
     * @param uri the URI to POST to
     * @param jsonBody the JSON for the POST body
     * @return a Response containing the status code and body.
     * @throws ParseException
     * @throws IOException
     */
    public static RestResponse doPost(String uri, String jsonBody) throws IOException {
        HttpPost method = new HttpPost(uri);
        method.addHeader(HttpHeaders.ACCEPT, ContentType.APPLICATION_JSON.getMimeType());
        method.addHeader(HttpHeaders.CONTENT_TYPE, ContentType.APPLICATION_JSON.getMimeType());
        method.setEntity(new StringEntity(jsonBody));

        log.debug("POST {}", uri);
        log.trace("\n" + jsonBody);
        CloseableHttpClient httpclient = httpClientBuilder.build();
        CloseableHttpResponse response = httpclient.execute(method);
        HttpEntity responseEntity = response.getEntity();
        // Some things reply with 204 No Content.
        String responseBody = responseEntity != null ? EntityUtils.toString(responseEntity) : null;
        httpclient.close();

        log.debug(response.getStatusLine().toString());
        log.trace("\n" + responseBody);

        return new RestResponse(response.getStatusLine().getStatusCode(), responseBody);
    }

    public static RestResponse doPost(String uri, Object object) throws IOException {
        return doPost(uri, GsonUtil.toJson(object));
    }

    public static RestResponse doDelete(String uri) throws IOException {
        HttpDelete method = new HttpDelete(uri);
        method.addHeader(HttpHeaders.ACCEPT, ContentType.APPLICATION_JSON.getMimeType());

        log.debug("DELETE {}", uri);
        CloseableHttpClient httpclient = httpClientBuilder.build();
        CloseableHttpResponse response = httpclient.execute(method);
        HttpEntity responseEntity = response.getEntity();
        // Some things reply with 204 No Content.
        String responseBody = responseEntity != null ? EntityUtils.toString(responseEntity) : null;
        httpclient.close();

        log.debug(response.getStatusLine().toString());
        log.trace("\n" + responseBody);

        return new RestResponse(response.getStatusLine().getStatusCode(), responseBody);
    }

    public static RestResponse doPut(String uri, String jsonBody) throws IOException {
        HttpPut method = new HttpPut(uri);
        method.addHeader(HttpHeaders.ACCEPT, ContentType.APPLICATION_JSON.getMimeType());
        method.addHeader(HttpHeaders.CONTENT_TYPE, ContentType.APPLICATION_JSON.getMimeType());
        method.setEntity(new StringEntity(jsonBody));

        log.debug("PUT {}", uri);
        log.trace("\n" + jsonBody);
        CloseableHttpClient httpclient = httpClientBuilder.build();
        CloseableHttpResponse response = httpclient.execute(method);
        HttpEntity responseEntity = response.getEntity();
        // Some things reply with 204 No Content.
        String responseBody = responseEntity != null ? EntityUtils.toString(responseEntity) : null;
        httpclient.close();

        log.debug(response.getStatusLine().toString());
        log.trace("\n" + responseBody);

        return new RestResponse(response.getStatusLine().getStatusCode(), responseBody);
    }

    public static RestResponse doPut(String uri, Object object) throws IOException {
        return doPut(uri, GsonUtil.toJson(object));
    }
}
