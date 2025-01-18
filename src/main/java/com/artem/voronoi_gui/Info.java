package com.artem.voronoi_gui;

import java.io.Serializable;
import java.util.ArrayList;

public class Info implements Serializable {
    public String username;
    public ArrayList<Integer[]> point_list;
    public ArrayList<Integer[]> line_list;
}
