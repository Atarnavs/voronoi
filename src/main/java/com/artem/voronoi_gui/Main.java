package com.artem.voronoi_gui;

import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        Connector connector = new Connector();
        ArrayList<Integer[]> points = new ArrayList<>();
        Area area = new Area(points);
//        Info info = connector.getLists();
//        for (Integer[] point: info.point_list) {
//            System.out.println(point[0] + ", " + point[1]);
//        }
//        area.setPointList(info.point_list);
    }
}