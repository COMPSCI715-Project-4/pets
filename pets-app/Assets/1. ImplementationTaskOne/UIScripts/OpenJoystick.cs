using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class OpenJoystick : MonoBehaviour
{
    public GameObject AddBtn;
    public GameObject CloseBtn;
    public GameObject controller;


    public void openController()
    {
        controller.SetActive(true);
        AddBtn.SetActive(false);
        CloseBtn.SetActive(true);
    }

    public void closeController()
    {
        controller.SetActive(false);
        AddBtn.SetActive(true);
        CloseBtn.SetActive(false);
    }
}
