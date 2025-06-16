package com.artem.voronoi_gui;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import utils.gson.GsonUtil;
import utils.http.HttpUtil;

import java.io.IOException;

public class Connector {
    private static final Logger log = LoggerFactory.getLogger(Connector.class);

    private final String engineUrl = "http://localhost:8080";
    public Info getLists() {
        try {
            var resp = HttpUtil.doGet(engineUrl+"/next_step");
            return GsonUtil.fromJson(resp.getBody(), Info.class);
        }
        catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
