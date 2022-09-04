#if UNITY_EDITOR
namespace UIWidgets
{
	using System;
	using System.Collections.Generic;
	using System.Reflection;
	using UnityEditor;
	using UnityEngine;
	using UnityEngine.Events;

	/// <summary>
	/// Customized ListView's editor.
	/// </summary>
	[CustomEditor(typeof(ListViewBase), true)]
	public class ListViewCustomBaseEditor : UIWidgetsMonoEditor
	{
		/// <summary>
		/// Is it ListViewCustom?
		/// </summary>
		protected bool IsListViewCustom = false;

		/// <summary>
		/// Is it TreeViewCustom?
		/// </summary>
		protected bool IsTreeViewCustom = false;

		/// <summary>
		/// Serialized properties.
		/// </summary>
		protected Dictionary<string, SerializedProperty> SerializedProperties = new Dictionary<string, SerializedProperty>();

		/// <summary>
		/// Serialized events.
		/// </summary>
		protected Dictionary<string, SerializedProperty> SerializedEvents = new Dictionary<string, SerializedProperty>();

		/// <summary>
		/// Properties.
		/// </summary>
		protected List<string> Properties = new List<string>()
		{
			"interactable",
			"disableScrollRect",
			"virtualization",
			"listType",
			"PrecalculateItemSizes",

			"customItems",
			"reversedOrder",
			"selectedIndex",
			"multipleSelect",
			"RangeMode",
			"direction",

			"scrollRect",
			"Container",
			"setContentSizeFitter",
			"defaultItem",
			"destroyDefaultItemsCache",

			"allowColoring",
			"defaultColor",
			"defaultBackgroundColor",
			"highlightedColor",
			"highlightedBackgroundColor",
			"selectedColor",
			"selectedBackgroundColor",
			"disabledColor",
			"FadeDuration",
			"KeepHighlight",
			"OnlyOneHighlighted",

			// other
			"Navigation",
			"ToggleOnNavigate",
			"ToggleOnSubmitCancel",

			"ScrollUnscaledTime",
			"ScrollMovement",
			"AutoScrollArea",
			"AutoScrollSpeed",
			"EndScrollDelay",
			"centerTheItems",
			"scrollInertiaUntilItemCenter",
			"ScrollInertia",
			"loopedList",
		};

		/// <summary>
		/// Properties indents.
		/// </summary>
		protected Dictionary<string, int> Indents = new Dictionary<string, int>()
		{
			{ "destroyDefaultItemsCache", 1 },
			{ "RangeMode", 1 },
			{ "disableScrollRect", 1 },
			{ "PrecalculateItemSizes", 1 },
			{ "setContentSizeFitter", 1 },
			{ "defaultColor", 1 },
			{ "defaultBackgroundColor", 1 },
			{ "highlightedColor", 1 },
			{ "highlightedBackgroundColor", 1 },
			{ "selectedColor", 1 },
			{ "selectedBackgroundColor", 1 },
			{ "disabledColor", 1 },
			{ "FadeDuration", 1 },
			{ "KeepHighlight", 1 },
			{ "OnlyOneHighlighted", 1 },
		};

		/// <summary>
		/// Scroll properties.
		/// </summary>
		protected List<string> ScrollFields = new List<string>()
		{
			"centerTheItems",
			"loopedList",
			"ScrollUnscaledTime",
			"ScrollMovement",
			"scrollInertiaUntilItemCenter",
			"ScrollInertia",
			"EndScrollDelay",
			"AutoScrollArea",
			"AutoScrollSpeed",
		};

		/// <summary>
		/// Coloring fields.
		/// </summary>
		protected List<string> ColoringFields = new List<string>()
		{
			"defaultColor",
			"defaultBackgroundColor",
			"highlightedColor",
			"highlightedBackgroundColor",
			"selectedColor",
			"selectedBackgroundColor",
			"disabledColor",
			"FadeDuration",
			"KeepHighlight",
			"OnlyOneHighlighted",
		};

		/// <summary>
		/// Events.
		/// </summary>
		protected List<string> Events = new List<string>()
		{
			"OnSelect",
			"OnDeselect",
			"OnSelectObject",
			"OnDeselectObject",
			"OnStartScrolling",
			"OnEndScrolling",
		};

		/// <summary>
		/// Exclude properties and events.
		/// </summary>
		protected List<string> Exclude = new List<string>()
		{
			"selectedIndices",
			"sort",
			"KeepSelection",

			// obsolete
			"LimitScrollValue",
			"FixHighlightItemUnderPointer",
		};

		static bool DetectGenericType(object instance, string name)
		{
			Type type = instance.GetType();
			while (type != null)
			{
				if (type.FullName.StartsWith(name, StringComparison.Ordinal))
				{
					return true;
				}

				type = type.BaseType;
			}

			return false;
		}

		/// <summary>
		/// Fill properties list.
		/// </summary>
		protected virtual void FillProperties()
		{
			var property = serializedObject.GetIterator();
			property.NextVisible(true);
			while (property.NextVisible(false))
			{
				AddProperty(property);
			}
		}

		/// <summary>
		/// Add property.
		/// </summary>
		/// <param name="property">Property.</param>
		protected void AddProperty(SerializedProperty property)
		{
			if (Exclude.Contains(property.name))
			{
				return;
			}

			if (Events.Contains(property.name) || Properties.Contains(property.name))
			{
				return;
			}

			if (IsEvent(property))
			{
				Events.Add(property.name);
			}
			else
			{
				Properties.Add(property.name);
			}
		}

		/// <summary>
		/// Is it event?
		/// </summary>
		/// <param name="property">Property</param>
		/// <returns>true if property is event; otherwise false.</returns>
		protected virtual bool IsEvent(SerializedProperty property)
		{
			var object_type = property.serializedObject.targetObject.GetType();
			var property_type = object_type.GetField(property.propertyPath, BindingFlags.NonPublic | BindingFlags.Public | BindingFlags.Instance);
			if (property_type == null)
			{
				return false;
			}

			return typeof(UnityEventBase).IsAssignableFrom(property_type.FieldType);
		}

		GUILayoutOption[] toggleOptions = new GUILayoutOption[] { GUILayout.ExpandWidth(true), GUILayout.Height(20) };

		/// <summary>
		/// Target.
		/// </summary>
		protected EditorConditional.TypeData DataSource;

		/// <summary>
		/// Init.
		/// </summary>
		protected virtual void OnEnable()
		{
			FillProperties();

			if (!IsListViewCustom)
			{
				IsListViewCustom = DetectGenericType(serializedObject.targetObject, "UIWidgets.ListViewCustom`2");
			}

			if (!IsTreeViewCustom)
			{
				IsTreeViewCustom = DetectGenericType(serializedObject.targetObject, "UIWidgets.TreeViewCustom`2");
			}

			if (IsTreeViewCustom)
			{
				Properties.Remove("customItems");
				Properties.Remove("selectedIndex");
				Properties.Remove("loopedList");
			}

			if (IsListViewCustom)
			{
				foreach (var p in Properties)
				{
					var property = serializedObject.FindProperty(p);
					if (property != null)
					{
						SerializedProperties[p] = property;
					}
				}

				foreach (var ev in Events)
				{
					var property = serializedObject.FindProperty(ev);
					if (property != null)
					{
						SerializedEvents[ev] = property;
					}
				}
			}
		}

		/// <summary>
		/// Toggle events block.
		/// </summary>
		protected bool ShowEvents;

		/// <summary>
		/// Toggle scroll properties block.
		/// </summary>
		protected bool ShowScrollProperties;

		void Upgrade()
		{
			foreach (var t in targets)
			{
				var lv = t as ListViewBase;
				if (lv != null)
				{
					lv.Upgrade();
				}
			}
		}

		/// <summary>
		/// Is property should be displayed.
		/// </summary>
		/// <param name="propertyName">Property name.</param>
		/// <returns>true if property should be displayed; otherwise false.</returns>
		protected bool IsShow(string propertyName)
		{
			if (propertyName == "PrecalculateItemSizes")
			{
				return IsVariableListType;
			}

			if (propertyName == "ItemsEvents")
			{
				return false;
			}

			if (propertyName == "RangeMode")
			{
				return SerializedProperties["multipleSelect"].boolValue;
			}

			if (ColoringFields.Contains(propertyName))
			{
				return SerializedProperties["allowColoring"].boolValue;
			}

			if (ScrollFields.Contains(propertyName))
			{
				return false;
			}

			return true;
		}

		/// <summary>
		/// List type.
		/// </summary>
		protected ListViewType ListType
		{
			get
			{
				return (ListViewType)SerializedProperties["listType"].enumValueIndex;
			}
		}

		/// <summary>
		/// Is TileView?
		/// </summary>
		protected bool IsTileView
		{
			get
			{
				return (ListType == ListViewType.TileViewWithFixedSize) || (ListType == ListViewType.TileViewWithVariableSize);
			}
		}

		/// <summary>
		/// Is variable list type?
		/// </summary>
		protected bool IsVariableListType
		{
			get
			{
				return (ListType == ListViewType.ListViewWithVariableSize) || (ListType == ListViewType.TileViewWithVariableSize);
			}
		}

		/// <summary>
		/// Draw inspector GUI.
		/// </summary>
		public override void OnInspectorGUI()
		{
			ValidateTargets();

			Upgrade();

			if (IsListViewCustom)
			{
				serializedObject.Update();

				foreach (var property_name in Properties)
				{
					if (!IsShow(property_name))
					{
						continue;
					}

					if (!SerializedProperties.TryGetValue(property_name, out var property))
					{
						continue;
					}

					if (!Indents.TryGetValue(property_name, out var indent))
					{
						indent = 0;
					}

					EditorGUI.indentLevel += indent;

					if (property_name == "customItems")
					{
						EditorGUILayout.PropertyField(property, new GUIContent("Data Source"), true);
					}
					else
					{
						EditorGUILayout.PropertyField(property, true);
					}

					EditorGUI.indentLevel -= indent;
				}

				ShowScrollProperties = GUILayout.Toggle(ShowScrollProperties, "Scroll Settings", EditorStyles.foldout, toggleOptions);

				if (ShowScrollProperties)
				{
					EditorGUI.indentLevel += 1;

					foreach (var field in ScrollFields)
					{
						if (field == "loopedList" && !IsTileView)
						{
							continue;
						}

						EditorGUILayout.PropertyField(SerializedProperties[field], true);
					}

					EditorGUI.indentLevel -= 1;
				}

				ShowEvents = GUILayout.Toggle(ShowEvents, "Events", EditorStyles.foldout, toggleOptions);

				if (ShowEvents)
				{
					foreach (var se in SerializedEvents)
					{
						EditorGUILayout.PropertyField(se.Value, true);
					}
				}

				EditorGUI.indentLevel += 1;
				EditorGUILayout.PropertyField(SerializedProperties["ItemsEvents"], new GUIContent("Items Events"), true);
				EditorGUI.indentLevel -= 1;

				serializedObject.ApplyModifiedProperties();

				ShowWarnings();
			}
			else
			{
				DrawDefaultInspector();
			}

			ValidateTargets();
		}

		/// <summary>
		/// Show warnings.
		/// </summary>
		protected void ShowWarnings()
		{
			var show_warning = false;

			foreach (var t in targets)
			{
				var type = t.GetType();
				var method = type.GetMethod("IsVirtualizationPossible", BindingFlags.NonPublic | BindingFlags.Public | BindingFlags.Instance);
				if (method != null)
				{
					var virtualization = (Func<bool>)Delegate.CreateDelegate(typeof(Func<bool>), t, method);
					show_warning |= !virtualization.Invoke();
				}
			}

			if (!show_warning)
			{
				return;
			}

			if (IsTileView || IsTreeViewCustom)
			{
				EditorGUILayout.HelpBox("Virtualization requires specified ScrollRect and Container should have EasyLayout component.", MessageType.Warning);
			}
			else
			{
				EditorGUILayout.HelpBox("Virtualization requires specified ScrollRect and Container should have EasyLayout or Horizontal or Vertical Layout Group component.", MessageType.Warning);
			}
		}
	}
}
#endif